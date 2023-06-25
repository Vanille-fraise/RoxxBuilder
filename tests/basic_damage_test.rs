use std::collections::HashMap;
use roxx_builder;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_calculation::DamageCalculation::{Average, Max, Min};
use roxx_builder::builder::attack_mod::damage_element::DamageElement;
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::attack_mod::damage_position::DamagePosition;
use roxx_builder::builder::attack_mod::damage_source::DamageSource;
use roxx_builder::builder::build_mod::build::Build;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::{Chance, DoMulti, DoPerArme, DoPerFinaux, DoPerSo, DoTerre, Force, Puissance};
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_slot::ItemSlot::{SlotAnneau1, SlotAnneau2};
use roxx_builder::builder::item_mod::stats::Stats;

#[test]
fn base_damage_test_with_and_without_damage() {
    let mut build: Build = Build::new();
    let mut item: Item = Item::default();
    let elem_damage = 10;
    item.stats.set_stat(&BaseStat::DoAir, elem_damage);

    let damage_line = DamageLine { damage_element: DamageElement::DamageAir, min_value: 10, max_value: 10 };
    let attack: Attack = Attack::new(vec![damage_line], vec![], DamageSource::Sort, DamagePosition::Distance, false, 0, Average);
    // println!("Attack - Brut: {:?}, Brut crit: {:?}", attack.brutality_damage(), attack.brutality_crit_damage());
    item.stats.reset_brutality(&attack);
    // println!("Item - Brut: {:?}", item.stats.brutality_stats());
    let evaluation_before = build.evaluate_build_damage(&attack);
    build.add_item(&item, SlotAnneau1);
    let evaluation_after = build.evaluate_build_damage(&attack);
    assert_eq!(evaluation_before, attack.damages().get(0).unwrap().min_value);
    assert_eq!(evaluation_after, evaluation_before + attack.damages().get(0).unwrap().min_value)
}

fn test_min_damage(attack: &mut Attack, build: &mut Build, expected: i64) {
    // build.recompute_all_stats();
    build.stats.reset_brutality(attack);
    attack.compute_brutality_damage();
    assert_eq!(build.evaluate_build_damage(&attack), expected)
}

fn test_damage_so_100_terre_melee(build: &mut Build, expected: i64) {
    test_min_damage(&mut Attack::new(vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], vec![], DamageSource::Sort, DamagePosition::Melee, false, 5, Max), build, expected);
}

#[test]
fn damage_with_stats_eau() {
    test_min_damage(&mut Attack::new(vec![DamageLine::new_fix(DamageElement::DamageEau, 15)], vec![], DamageSource::Sort, DamagePosition::Distance, false, 0, Average), &mut Build::new_with_stats(Stats::from_map_stats(HashMap::from([(Chance, 300)]).iter())), 60);
}

#[test]
fn damage_with_stats_and_damage_and_puissance_and_do_multi_terre() {
    test_min_damage(&mut Attack::new(vec![DamageLine::new_fix(DamageElement::DamageTerre, 80)], vec![], DamageSource::Sort, DamagePosition::Distance, false, 0, Min),
                    &mut Build::new_with_stats(Stats::from_map_stats(HashMap::from([(Force, 1200), (DoTerre, 120), (Puissance, 300), (DoMulti, 20)]).iter())), 1420);
}

#[test]
fn damage_with_multiple_items() {
    test_damage_so_100_terre_melee(&mut Build::new_with_item_map(
        &HashMap::from([
            (SlotAnneau1, &Item::new_with_stats(Stats::from_map_stats(HashMap::from([(Force, 300)]).iter()))),
            (SlotAnneau2, &Item::new_with_stats(Stats::from_map_stats(HashMap::from([(Force, 600), (DoTerre, 200), (DoPerSo, 10), (DoPerFinaux, 10), (DoPerArme, 30)]).iter())))])),
                                   1452);
}

#[test]
fn damage_with_same_spot_items() {
    test_damage_so_100_terre_melee(&mut Build::new_with_item_map(&HashMap::from([(SlotAnneau1, &Item::new_with_stats(Stats::from_map_stats(HashMap::from([(Force, 300)]).iter()))), (SlotAnneau1, &Item::new_with_stats(Stats::from_map_stats(HashMap::from([(Force, 600), (DoTerre, 200), (DoPerSo, 10), (DoPerFinaux, 10), (DoPerArme, 30)]).iter())))])), 1089);
}
