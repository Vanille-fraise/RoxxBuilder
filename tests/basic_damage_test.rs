use std::collections::HashMap;
use roxx_builder;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_element::DamageElement;
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::attack_mod::damage_position::DamagePosition;
use roxx_builder::builder::attack_mod::damage_source::DamageSource;
use roxx_builder::builder::build_mod::build::Build;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::{Chance, DoMulti, DoTerre, Force, Puissance};
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_type::ItemType;

#[test]
fn base_damage_test_with_and_without_damage() {
    let mut build: Build = Build::new();
    let mut item: Item = Item::new(ItemType::Anneau);
    let elem_damage = 10;
    item.stats.insert(BaseStat::DoAir, elem_damage);

    let damage_line = DamageLine { damage_element: DamageElement::DamageAir, value: 10 };
    let attack: Attack = Attack::new(vec![damage_line], DamageSource::Sort, DamagePosition::Distance);

    let evaluation_before = build.evaluate_attack(&attack);
    build.add_item(&item);
    let evaluation_after = build.evaluate_attack(&attack);

    assert_eq!(evaluation_before, attack.damages.get(0).unwrap().value);
    assert_eq!(evaluation_after, evaluation_before + attack.damages.get(0).unwrap().value)
}

fn test_damage(attack: &Attack, build: &Build, expected: i32) {
    assert_eq!(build.evaluate_attack(attack), expected)
}

#[test]
fn damage_with_stats_eau() {
    test_damage(&Attack::new(vec![DamageLine { damage_element: DamageElement::DamageEau, value: 15 }], DamageSource::Sort, DamagePosition::Distance), &Build::new_with(HashMap::from([(Chance, 300)])), 60);
}

#[test]
fn damage_with_stats_and_damage_and_puissance_and_do_multi_terre() {
    test_damage(&Attack::new(vec![DamageLine { damage_element: DamageElement::DamageTerre, value: 80 }], DamageSource::Sort, DamagePosition::Distance), &Build::new_with(HashMap::from([(Force, 1200), (DoTerre, 120), (Puissance, 300), (DoMulti, 20)])), 1420);
}