use std::collections::{BTreeMap, HashMap};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use strum::IntoEnumIterator;

fn add_base_stats_hashmap(nb_stats: u32, nb_loops: u32) {
    let mut n_s = nb_stats;
    let mut stats: HashMap<BaseStat, i64> = HashMap::default();
    let mut stats_to_add: HashMap<BaseStat, i64> = HashMap::default();
    for bst in BaseStat::iter() {
        if n_s > 0 {
            n_s -= 1;
            stats_to_add.insert(bst, 10);
        }
        stats.insert(bst, 10);
    }
    for _nb_loop in 0..nb_loops {
        for st in stats_to_add.iter() {
            *stats.entry(*st.0).or_insert(0) += st.1
        }
    }
}

fn add_base_stats_btree(nb_stats: u32, nb_loops: u32) {
    let mut stats: BTreeMap<BaseStat, i64> = BTreeMap::default();
    let mut stats_to_add: BTreeMap<BaseStat, i64> = BTreeMap::default();
    let mut n_s = nb_stats;
    for bst in BaseStat::iter() {
        if n_s > 0 {
            n_s -= 1;
            stats_to_add.insert(bst, 10);
        }
        stats.insert(bst, 10);
    }
    for _nb_loop in 0..nb_loops {
        for st in stats_to_add.iter() {
            *stats.entry(*st.0).or_insert(10) += st.1
        }
    }
}

fn add_base_stats_array(nb_stats: u32, nb_loops: u32) {
    let mut stats: [i64; 256] = [10; 256];
    let to_add: Vec<usize> = (0..nb_stats as usize).collect();
    stats = stats;
    let mut stats_to_add: [i64; 256] = [10; 256];
    for _nb_loop in 0..nb_loops {
        for st in to_add.iter() {
            stats_to_add[*st] += stats[*st];
        }
    }
}

fn add_base_stats_vec(nb_stats: u32, nb_loops: u32) {
    let mut stats: Vec<i64> = vec![];
    let mut stats_to_add: Vec<i64> = vec![];
    let mut n_s = nb_stats;
    for _bst in BaseStat::iter() {
        if n_s > 0 {
            n_s -= 1;
            stats_to_add.push(10);
        }
        stats.push(10);
    }
    for _nb_loop in 0..nb_loops {
        for st in 0..stats_to_add.len() {
            stats[st] += stats_to_add[st];
        }
    }
}


fn bench_stats(c: &mut Criterion) {
    let mut group = c.benchmark_group("Stats");
    for val in [
        (2, 10000), (8, 10000), (40, 10000), (120, 10000)].iter()
    {
        group.bench_with_input(BenchmarkId::new("HashMap", val.0 + val.1), val,
                               |b, val| b.iter(|| add_base_stats_hashmap(val.0, val.1)));
        group.bench_with_input(BenchmarkId::new("BTree", val.0 + val.1), val,
                               |b, val| b.iter(|| add_base_stats_btree(val.0, val.1)));
        group.bench_with_input(BenchmarkId::new("Array", val.0 + val.1), val,
                               |b, val| b.iter(|| add_base_stats_array(val.0, val.1)));
        group.bench_with_input(BenchmarkId::new("Vector", val.0 + val.1), val,
                               |b, val| b.iter(|| add_base_stats_vec(val.0, val.1)));
    }
    group.finish();
}

criterion_group!(benches, bench_stats);
criterion_main!(benches);