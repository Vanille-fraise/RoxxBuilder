//! Profiling harness for the graph search (`GraphLooker`) on real dofus-db data
//! with the COCO spell (same scenario as `real_item_coherent_result_test`).
//!
//! Run from the repository root:
//! ```sh
//! RUSTFLAGS="-C force-frame-pointers=yes -Cdebuginfo=1" cargo run --release --example profile_coco
//! ```
//! Frame pointers are required by pprof's stack walking, debuginfo gives file:line in the SVG.
//! Output: `target/flamegraph-coco.svg` (open in a browser).

use std::fs::File;
use std::time::Instant;

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

#[actix_rt::main]
async fn main() {
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

    println!("Search wall time: {search_elapsed:?}");
    println!("Builds evaluated: {}", res.builds_evaluated);
    println!("Builds considered: {}", res.builds_considered);
    println!("Builds per sec: {}", res.builds_evaluated / (SEARCH_TIME_MILLI / 1000) as i64);
    println!("Eval: {}", res.eval);
    for (key, value) in &res.additional_data {
        println!("{key}: {value}");
    }
}
