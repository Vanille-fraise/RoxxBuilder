use roxx_builder::builder::build_mod::build::Build;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::Force;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_slot::ItemSlot::{SlotAnneau1, SlotAnneau2, SlotBottes, SlotCeinture};
use roxx_builder::builder::item_mod::set::Set;
use std::collections::HashMap;
use roxx_builder::builder::item_mod::stats::Stats;

fn set_stat_test(build: &Build, stat: BaseStat, expect: i64, len: usize, set_id: i64, set_number: usize) {
    assert_eq!(build.sets.len(), len);
    assert_eq!(build.sets.get(&set_id), Some(&set_number));
    assert_eq!(build.stats.get_stat(&stat), expect);
}

#[test]
fn simple_set_test() {
    let (mut item_1, mut item_2) = (Item::default(), Item::default());
    item_1.set_id = 1;
    item_2.set_id = 1;
    let set = Set::new(1, vec![Stats::from_map_stats(HashMap::from([(Force, 100)]).iter()), Stats::from_map_stats(HashMap::from([(Force, 1000)]).iter())]);
    item_1.set = Some(&set);
    item_2.set = Some(&set);
    let mut build = Build::new();
    build.add_item(&item_1, SlotAnneau2);
    set_stat_test(&build, Force, 100, 1, 1, 0);
    build.add_item(&item_2, SlotAnneau1);
    set_stat_test(&build, Force, 1000, 1, 1, 1);
}

#[test]
fn replacing_set_test() {
    let (mut item_1, mut item_2) = (Item::default(), Item::default());
    item_1.set_id = 1;
    item_2.set_id = 1;
    let set = Set::new(1, vec![Stats::from_map_stats(HashMap::from([(Force, 0)]).iter()), Stats::from_map_stats(HashMap::from([(Force, 1000)]).iter())]);
    item_1.set = Some(&set);
    item_2.set = Some(&set);
    let mut build = Build::new();
    build.add_item(&item_1, SlotAnneau1);
    set_stat_test(&build, Force, 0, 1, 1, 0);
    build.add_item(&item_2, SlotAnneau1);
    set_stat_test(&build, Force, 0, 1, 1, 0);
}

#[test]
fn multiple_set_test() {
    let (mut item_1, mut item_2) = (Item::default(), Item::default());
    item_1.set_id = 1;
    item_2.set_id = 1;
    let set_1 = Set::new(1, vec![Stats::from_map_stats(HashMap::from([(Force, 0)]).iter()), Stats::from_map_stats(HashMap::from([(Force, 1000)]).iter())]);
    item_1.set = Some(&set_1);
    item_2.set = Some(&set_1);

    let (mut item_3, mut item_4) = (Item::default(), Item::default());
    item_3.set_id = 2;
    item_4.set_id = 2;
    let set_2 = Set::new(2, vec![Stats::from_map_stats(HashMap::from([(Force, 500)]).iter()), Stats::from_map_stats(HashMap::from([(Force, 1500)]).iter())]);
    item_3.set = Some(&set_2);
    item_4.set = Some(&set_2);
    let mut build = Build::new();
    build.add_item(&item_1, SlotAnneau1);
    set_stat_test(&build, Force, 0, 1, 1, 0);
    build.add_item(&item_2, SlotAnneau2);
    set_stat_test(&build, Force, 1000, 1, 1, 1);

    build.add_item(&item_3, SlotBottes);
    set_stat_test(&build, Force, 1500, 2, 2, 0);

    build.add_item(&item_4, SlotCeinture);
    set_stat_test(&build, Force, 2500, 2, 2, 1);
}

#[test]
fn removing_set_test() {
    let (mut item_1, mut item_2) = (Item::default(), Item::default());
    item_1.set_id = 1;
    item_2.set_id = 1;
    let set = Set::new(1, vec![Stats::from_map_stats(HashMap::from([(Force, 100)]).iter()), Stats::from_map_stats(HashMap::from([(Force, 1000)]).iter())]);
    item_1.set = Some(&set);
    item_2.set = Some(&set);

    let mut build = Build::new();
    build.add_item(&item_1, SlotAnneau2);
    set_stat_test(&build, Force, 100, 1, 1, 0);
    build.add_item(&item_2, SlotAnneau1);
    set_stat_test(&build, Force, 1000, 1, 1, 1);

    build.remove_item(&SlotAnneau1);
    set_stat_test(&build, Force, 100, 1, 1, 0);

    build.remove_item(&SlotAnneau2);
    assert_eq!(build.sets.len(), 0);
    assert_eq!(build.sets.get(&1), None);
    assert_eq!(build.stats.get_stat(&Force), 0);
}