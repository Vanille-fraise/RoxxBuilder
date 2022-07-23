use std::collections::HashMap;
use roxx_builder;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_element::DamageElement;
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::attack_mod::damage_position::DamagePosition;
use roxx_builder::builder::attack_mod::damage_source::DamageSource;
use roxx_builder::builder::build_mod::build::Build;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::{Chance, DoMulti, DoPerArme, DoPerFinaux, DoPerSo, DoTerre, Force, Puissance};
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_slot::ItemSlot::{SlotAnneau1, SlotAnneau2, SlotBottes, SlotCeinture};
use roxx_builder::builder::item_mod::set::Set;

#[test]
fn base_damage_test_with_and_without_damage() {
    let mut build: Build = Build::new();
    let mut item: Item = Item::default();
    let elem_damage = 10;
    item.stats.insert(BaseStat::DoAir, elem_damage);

    let damage_line = DamageLine { damage_element: DamageElement::DamageAir, min_value: 10, max_value: 10 };
    let attack: Attack = Attack::new(vec![damage_line], vec![], DamageSource::Sort, DamagePosition::Distance, false, 0);

    let evaluation_before = build.evaluate_attack(&attack).0;
    build.add_item(&item, SlotAnneau1, false);
    let evaluation_after = build.evaluate_attack(&attack).0;

    assert_eq!(evaluation_before, attack.damages.get(0).unwrap().min_value);
    assert_eq!(evaluation_after, evaluation_before + attack.damages.get(0).unwrap().min_value)
}

fn test_min_damage(attack: &Attack, build: &Build, expected: i64) {
    assert_eq!(build.evaluate_attack(attack).0, expected)
}

fn test_damage_so_100_terre_melee(build: &Build, expected: i64) {
    test_min_damage(&Attack::new(vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], vec![], DamageSource::Sort, DamagePosition::Melee, false, 0), build, expected);
}

#[test]
fn damage_with_stats_eau() {
    test_min_damage(&Attack::new(vec![DamageLine::new_fix(DamageElement::DamageEau, 15)], vec![], DamageSource::Sort, DamagePosition::Distance, false, 0), &Build::new_with(HashMap::from([(Chance, 300)])), 60);
}

#[test]
fn damage_with_stats_and_damage_and_puissance_and_do_multi_terre() {
    test_min_damage(&Attack::new(vec![DamageLine::new_fix(DamageElement::DamageTerre, 80)], vec![], DamageSource::Sort, DamagePosition::Distance, false, 0), &Build::new_with(HashMap::from([(Force, 1200), (DoTerre, 120), (Puissance, 300), (DoMulti, 20)])), 1420);
}

#[test]
fn damage_with_multiple_items() {
    test_damage_so_100_terre_melee(&Build::new_with_items(HashMap::from([(SlotAnneau1, &Item::new_with_stats(HashMap::from([(Force, 300)]))), (SlotAnneau2, &Item::new_with_stats(HashMap::from([(Force, 600), (DoTerre, 200), (DoPerSo, 10), (DoPerFinaux, 10), (DoPerArme, 30)])))])), 1452);
}

#[test]
fn damage_with_same_spot_items() {
    test_damage_so_100_terre_melee(&Build::new_with_items(HashMap::from([(SlotAnneau1, &Item::new_with_stats(HashMap::from([(Force, 300)]))), (SlotAnneau1, &Item::new_with_stats(HashMap::from([(Force, 600), (DoTerre, 200), (DoPerSo, 10), (DoPerFinaux, 10), (DoPerArme, 30)])))])), 1089);
}

fn set_stat_test(build: &Build, stat: BaseStat, expect: i64, len: usize, set_id: i64, set_number: usize) {
    assert_eq!(build.sets.len(), len);
    assert_eq!(build.sets.get(&set_id), Some(&set_number));
    assert_eq!(*build.stats.get(&stat).unwrap(), expect);
}

#[test]
fn simple_set_test() {
    let (mut item_1, mut item_2) = (Item::default(), Item::default());
    item_1.set_id = 1;
    item_2.set_id = 1;
    let set = Set::new(1, vec![HashMap::from([(Force, 100)]), HashMap::from([(Force, 1000)])]);
    item_1.set = Some(&set);
    item_2.set = Some(&set);
    let mut build = Build::new();
    build.add_item(&item_1, SlotAnneau2, false);
    set_stat_test(&build, Force, 100, 1, 1, 0);
    build.add_item(&item_2, SlotAnneau1, false);
    set_stat_test(&build, Force, 1000, 1, 1, 1);
}

#[test]
fn replacing_set_test() {
    let (mut item_1, mut item_2) = (Item::default(), Item::default());
    item_1.set_id = 1;
    item_2.set_id = 1;
    let set = Set::new(1, vec![HashMap::from([(Force, 0)]), HashMap::from([(Force, 1000)])]);
    item_1.set = Some(&set);
    item_2.set = Some(&set);
    let mut build = Build::new();
    build.add_item(&item_1, SlotAnneau1, false);
    set_stat_test(&build, Force, 0, 1, 1, 0);
    build.add_item(&item_2, SlotAnneau1, false);
    set_stat_test(&build, Force, 0, 1, 1, 0);
}

#[test]
fn multiple_set_test() {
    let (mut item_1, mut item_2) = (Item::default(), Item::default());
    item_1.set_id = 1;
    item_2.set_id = 1;
    let set_1 = Set::new(1, vec![HashMap::from([(Force, 0)]), HashMap::from([(Force, 1000)])]);
    item_1.set = Some(&set_1);
    item_2.set = Some(&set_1);

    let (mut item_3, mut item_4) = (Item::default(), Item::default());
    item_3.set_id = 2;
    item_4.set_id = 2;
    let set_2 = Set::new(2, vec![HashMap::from([(Force, 500)]), HashMap::from([(Force, 1500)])]);
    item_3.set = Some(&set_2);
    item_4.set = Some(&set_2);
    let mut build = Build::new();
    build.add_item(&item_1, SlotAnneau1, false);
    set_stat_test(&build, Force, 0, 1, 1, 0);
    build.add_item(&item_2, SlotAnneau2, false);
    set_stat_test(&build, Force, 1000, 1, 1, 1);

    build.add_item(&item_3, SlotBottes, false);
    set_stat_test(&build, Force, 1500, 2, 2, 0);

    build.add_item(&item_4, SlotCeinture, false);
    set_stat_test(&build, Force, 2500, 2, 2, 1);
}

#[test]
fn removing_set_test() {
    let (mut item_1, mut item_2) = (Item::default(), Item::default());
    item_1.set_id = 1;
    item_2.set_id = 1;
    let set = Set::new(1, vec![HashMap::from([(Force, 100)]), HashMap::from([(Force, 1000)])]);
    item_1.set = Some(&set);
    item_2.set = Some(&set);

    let mut build = Build::new();
    build.add_item(&item_1, SlotAnneau2, false);
    set_stat_test(&build, Force, 100, 1, 1, 0);
    build.add_item(&item_2, SlotAnneau1, false);
    set_stat_test(&build, Force, 1000, 1, 1, 1);

    build.remove_item(&SlotAnneau1);
    set_stat_test(&build, Force, 100, 1, 1, 0);

    build.remove_item(&SlotAnneau2);
    assert_eq!(build.sets.len(), 0);
    assert_eq!(build.sets.get(&1), None);
    assert_eq!(*build.stats.get(&Force).unwrap(), 0);

}