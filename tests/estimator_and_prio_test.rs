use std::collections::HashMap;
use rand::seq::SliceRandom;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_type::ItemType;
use roxx_builder::builder::algorithm_mod::item_estimator::ItemEstimator;
use roxx_builder::builder::algorithm_mod::roxx_build_finder::RoxxBuildFinder;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_element::DamageElement;
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::attack_mod::damage_position::DamagePosition;
use roxx_builder::builder::attack_mod::damage_source::DamageSource;
use roxx_builder::builder::data_mod::data_container::DataContainer;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::item_type::ItemType::{Amulette, Anneau, Arc, Bottes, Bouclier, Cape, Ceinture, Chapeau, Dofus, Familier};

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref COCO : Attack = Attack::new(vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], DamageSource::Sort, DamagePosition::Melee, true, 25);
}

// #[test]
#[allow(dead_code)]
fn full_crap_find_right_comb() {
    let types: Vec<ItemType> = vec![Amulette, Arc, Anneau, Anneau, Ceinture, Bottes, Chapeau, Cape, Familier, Dofus, Dofus, Dofus, Dofus, Dofus, Dofus, Bouclier];
    let mut dc = DataContainer::new();
    for t in types.iter() {
        let mut item = Item::new_with_stats(HashMap::from([(BaseStat::DoMulti, 100), (BaseStat::Puissance, 1000)]));
        item.item_type = t.clone();
        dc.items.push(item);
    }
    for i in 0..5000usize {
        let mut item = Item::new_with_stats(HashMap::from([(BaseStat::DoMulti, (i % 13) as i64), (BaseStat::Puissance, (i % 11) as i64)]));
        item.item_type = types.get(i % types.len()).unwrap().clone();
        dc.items.push(item);
    }
    let mut rng = rand::thread_rng();
    dc.items.shuffle(&mut rng);
    let mut rb = RoxxBuildFinder::new(&dc);
    let time_limit = 500_000_000u128;
    rb.time_limit = time_limit;
    rb.estimator = Some(ItemEstimator::roxx_based_estimation);
    let res = rb.find_build(&COCO);
    let d = time_limit / 1_000_000;
    print!("Res : {:?}", res);
    assert!(res.search_time.mul_f64(0.8).as_millis() < d);
    assert!(res.search_time.as_millis() > d);
    assert_eq!(res.average, 18600);
}

#[test]
fn time_limit_test() {}
