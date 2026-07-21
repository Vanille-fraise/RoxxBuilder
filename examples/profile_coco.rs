//! Profiling harness for the graph search (`GraphLooker`) on real dofus-db data
//! with the COCO spell (same scenario as `real_item_coherent_result_test`).
//!
//! Run from the repository root:
//! ```sh
//! RUSTFLAGS="-C force-frame-pointers=yes -Cdebuginfo=1" cargo run --release --example profile_coco -- <label>
//! ```
//! Frame pointers are required by pprof's stack walking, debuginfo gives file:line in the SVG.
//!
//! Every run is recorded for comparison across optimizations (see AGENTS.md):
//! - flame graph written to `target/flamegraph-coco.svg` and copied to
//!   `benchmarks/flamegraph-<label>-<datetime>.svg`
//! - one row appended to `benchmarks/results.csv`

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

use roxx_builder::builder::algorithm_mod::roxx_builder_graph::GraphLooker;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_calculation::DamageCalculation;
use roxx_builder::builder::attack_mod::damage_element::DamageElement;
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::attack_mod::damage_position::DamagePosition;
use roxx_builder::builder::attack_mod::damage_source::DamageSource;
use roxx_builder::builder::data_mod::data_manager::DataManager;
use roxx_builder::builder::player_mod::player::SearchOptions;

const SEARCH_TIME_MILLI: u128 = 10_000;
const SAMPLE_FREQUENCY_HZ: i32 = 1000;
const OUTPUT_FILE: &str = "target/flamegraph-coco.svg";
const BENCHMARKS_DIR: &str = "benchmarks";
const RESULTS_CSV: &str = "benchmarks/results.csv";
const CSV_HEADER: &str = "datetime_utc,git_commit,label,search_time_ms,builds_evaluated,builds_considered,builds_per_sec,eval,preparation_ms,final_neighbors,pprof_samples,flamegraph_file";

#[actix_rt::main]
async fn main() {
    let label = std::env::args()
        .nth(1)
        .map(|s| sanitize_label(&s))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unlabeled".to_string());

    // Same attack as REAL_COCO in tests/roxx_builder_graph_test.rs
    let coco = Attack::new(
        vec![DamageLine::new(DamageElement::DamageTerre, 81, 100)],
        vec![DamageLine::new(DamageElement::DamageTerre, 97, 120)],
        DamageSource::Sort,
        DamagePosition::Melee,
        true,
        25,
        DamageCalculation::Max,
    );
    let black_list = vec![
        "Surpuissant Chacha de Combat (MJ)".to_string(),
        "Annobusé de Maître Jarbo".to_string(),
        "Petit Chacha de Combat (MJ)".to_string(),
    ];

    let load_start = Instant::now();
    let mut dc = DataManager::retrieve_data().await;
    dc.clear_unknown_type_items();
    // Seeded shuffle: same conditions as the test, but reproducible.
    dc.items.shuffle(&mut StdRng::seed_from_u64(42));
    dc.link_sub_data();
    println!("Data container loaded in {:?} ({} items)", load_start.elapsed(), dc.items.len());

    let mut search_option = SearchOptions::empty();
    search_option.set_search_time_milli(SEARCH_TIME_MILLI);
    search_option.set_black_list(black_list);

    let guard = pprof::ProfilerGuard::new(SAMPLE_FREQUENCY_HZ)
        .expect("failed to start pprof profiler");
    let search_start = Instant::now();
    let res = GraphLooker::search(&dc, &coco, &search_option);
    let search_elapsed = search_start.elapsed();

    match guard.report().build() {
        Ok(report) => {
            let mut file = File::create(OUTPUT_FILE).expect("failed to create flamegraph file");
            report.flamegraph(&mut file).expect("failed to write flamegraph");
            println!("Flamegraph written to {OUTPUT_FILE}");
        }
        Err(e) => eprintln!("Failed to build pprof report: {e}"),
    }

    let builds_per_sec = res.builds_evaluated / (SEARCH_TIME_MILLI / 1000) as i64;
    println!("Search wall time: {search_elapsed:?}");
    println!("Builds evaluated: {}", res.builds_evaluated);
    println!("Builds considered: {}", res.builds_considered);
    println!("Builds per sec: {builds_per_sec}");
    println!("Eval: {}", res.eval);
    for (key, value) in &res.additional_data {
        println!("{key}: {value}");
    }

    let preparation_ms = res.additional_data.get("Preparation time")
        .map(|s| parse_duration_ms(s)).unwrap_or(-1.0);
    let final_neighbors = res.additional_data.get("Final neighbors size")
        .and_then(|s| s.parse::<i64>().ok()).unwrap_or(-1);
    save_run(&label, &res, builds_per_sec, preparation_ms, final_neighbors);
}

/// Records the run: copies the flame graph into `benchmarks/` and appends a CSV row.
fn save_run(label: &str, res: &roxx_builder::builder::build_mod::build_search_result::BuildSearchResult,
            builds_per_sec: i64, preparation_ms: f64, final_neighbors: i64) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let datetime = format_datetime(now);
    let flamegraph_name = format!("flamegraph-{label}-{datetime}.svg");

    if let Err(e) = std::fs::create_dir_all(BENCHMARKS_DIR) {
        eprintln!("Could not create {BENCHMARKS_DIR}: {e}");
        return;
    }
    if let Err(e) = std::fs::copy(OUTPUT_FILE, format!("{BENCHMARKS_DIR}/{flamegraph_name}")) {
        eprintln!("Could not copy flamegraph to {BENCHMARKS_DIR}: {e}");
        return;
    }
    let pprof_samples = std::fs::read_to_string(OUTPUT_FILE).ok()
        .and_then(|c| c.split("total_samples=\"").nth(1)?.split('"').next()?.parse::<u64>().ok())
        .unwrap_or(0);

    let row = format!(
        "{},{},{},{},{},{},{},{},{:.2},{},{},{}",
        datetime, git_commit(), label, SEARCH_TIME_MILLI, res.builds_evaluated,
        res.builds_considered, builds_per_sec, res.eval, preparation_ms,
        final_neighbors, pprof_samples, flamegraph_name,
    );
    let write_header = std::fs::metadata(RESULTS_CSV).map(|m| m.len() == 0).unwrap_or(true);
    match OpenOptions::new().create(true).append(true).open(RESULTS_CSV) {
        Ok(mut f) => {
            if write_header { let _ = writeln!(f, "{CSV_HEADER}"); }
            let _ = writeln!(f, "{row}");
            println!("Results appended to {RESULTS_CSV} (label: {label})");
        }
        Err(e) => eprintln!("Could not append to {RESULTS_CSV}: {e}"),
    }
}

fn sanitize_label(label: &str) -> String {
    label.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == '+' { c } else { '-' }).collect()
}

fn git_commit() -> String {
    let hash = std::process::Command::new("git").args(["rev-parse", "--short", "HEAD"]).output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let dirty = std::process::Command::new("git").args(["status", "--porcelain"]).output().ok()
        .map(|o| !o.stdout.is_empty()).unwrap_or(false);
    if dirty { format!("{hash}-dirty") } else { hash }
}

/// Parses a `std::time::Duration` debug string (e.g. "15.393709ms") into milliseconds.
fn parse_duration_ms(debug: &str) -> f64 {
    let (value, unit) = debug.split_at(debug.find(|c: char| c.is_alphabetic()).unwrap_or(debug.len()));
    let value: f64 = value.parse().unwrap_or(-1.0);
    match unit {
        "ns" => value / 1_000_000.0,
        "µs" | "us" => value / 1_000.0,
        "ms" => value,
        "s" => value * 1_000.0,
        _ => -1.0,
    }
}

/// Formats epoch seconds as `YYYY-MM-DD_HH-MM-SS` (UTC, no external deps).
fn format_datetime(epoch_secs: u64) -> String {
    let secs = epoch_secs % 86400;
    let (h, mi, s) = (secs / 3600, (secs % 3600) / 60, secs % 60);
    // Howard Hinnant's civil-from-days algorithm.
    let z = (epoch_secs / 86400) as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let yr = if mo <= 2 { y + 1 } else { y };
    format!("{yr:04}-{mo:02}-{d:02}_{h:02}-{mi:02}-{s:02}")
}
