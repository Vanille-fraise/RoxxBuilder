use std::collections::HashMap;
use rand::seq::SliceRandom;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_type::ItemType;
use roxx_builder::builder::algorithm_mod::roxx_build_finder::RoxxBuildFinder;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_element::DamageElement;
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::attack_mod::damage_position::DamagePosition;
use roxx_builder::builder::attack_mod::damage_source::DamageSource;
use roxx_builder::builder::attack_mod::damage_calculation::DamageCalculation;
use roxx_builder::builder::data_mod::data_container::DataContainer;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::item_type::ItemType::{Amulette, Anneau, Arc, Bottes, Bouclier, Cape, Ceinture, Chapeau, Dofus, Familier, Prysmaradite, Trophee};
use roxx_builder::builder::item_mod::stats::Stats;

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref COCO : Attack = Attack::new(vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], DamageSource::Sort, DamagePosition::Melee, true, 25, DamageCalculation::Average);
}

#[test]
fn full_crap_find_right_comb() {
    let types: Vec<ItemType> = vec![Amulette, Arc, Ceinture, Bottes, Chapeau, Cape, Familier, Prysmaradite, Trophee, Dofus, Dofus, Trophee, Dofus, Bouclier, Anneau];
    let mut dc = DataContainer::new();
    for t in types.iter() {
        let mut item = Item::new_with_stats(Stats::from_map_stats(HashMap::from([(BaseStat::DoMulti, 10), (BaseStat::Puissance, 100)]).iter()));
        item.item_type = t.clone();
        item.name = "Best".to_string();
        dc.items.push(item);
    }
    for i in 0..5000usize {
        let mut item = Item::new_with_stats(Stats::from_map_stats(HashMap::from([(BaseStat::DoMulti, (i % 3) as i64), (BaseStat::Puissance, (i % 5) as i64)]).iter()));
        item.item_type = types.get(i % types.len()).unwrap().clone();
        dc.items.push(item);
    }
    let mut rng = rand::thread_rng();
    dc.items.shuffle(&mut rng);
    let mut rb = RoxxBuildFinder::new(dc, COCO.clone());
    let time_limit = 5_000_000_000u128;
    rb.time_limit = time_limit;
    rb.track_data = false;
    let res = rb.find_build();
    let d = time_limit / 1_000_000;
    println!("Build evaluated: {}", res.builds_evaluated);
    if rb.track_data { println!("Duplicated build tested : {:?}", res.additional_data.get("spares")); }
    assert!(res.search_time.mul_f64(0.8).as_millis() < d);
    assert_eq!(res.eval, 1860);
}

#[test]
fn time_limit_test() {}
