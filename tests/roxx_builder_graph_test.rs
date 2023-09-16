use std::collections::HashMap;
use rand::seq::SliceRandom;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_type::ItemType;
use roxx_builder::builder::algorithm_mod::roxx_builder_graph::GraphLooker;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_element::DamageElement;
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::attack_mod::damage_position::DamagePosition;
use roxx_builder::builder::attack_mod::damage_source::DamageSource;
use roxx_builder::builder::attack_mod::damage_calculation::DamageCalculation;
use roxx_builder::builder::build_mod::player::SearchOptions;
use roxx_builder::builder::data_mod::data_container::DataContainer;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::{BrutaliteSevere, Force};
use roxx_builder::builder::item_mod::item_type::ItemType::{Amulette, Anneau, Arc, Bottes, Bouclier, Cape, Ceinture, Chapeau, Dofus, Familier, Prysmaradite, Trophee};
use roxx_builder::builder::item_mod::stats::Stats;

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref COCO : Attack = Attack::new(vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], DamageSource::Sort, DamagePosition::Melee, true, 25, DamageCalculation::Average);
}

#[test]
fn empty_test() {
    let dc = DataContainer::new();
    let res = GraphLooker::search(&dc, &COCO, &SearchOptions::empty());
    assert_eq!(res.builds_evaluated, 0);
    assert_eq!(res.build.stats, Stats::new_empty());
}

#[test]
fn one_item_test() {
    let mut dc = DataContainer::new();
    let mut item = Item::empty();
    item.stats.set_stat(&Force, 900);
    dc.items.push(item);
    let res = GraphLooker::search(&dc, &COCO, &SearchOptions::empty());
    assert_eq!(res.eval, 1000);
    assert_eq!(res.builds_evaluated, 1);
}


#[test]
fn practical_test() {
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
    let time_limit = 3_000;
    let mut opt = SearchOptions::empty();
    opt.set_search_time_milli(time_limit);
    let res = GraphLooker::search(&dc, &COCO, &opt);
    println!("Build evaluated: {}", res.builds_evaluated);
    println!("Build per sec: {}", res.builds_evaluated / (time_limit / 1000) as i64);
    assert_eq!(res.eval, 1860);
}


#[test]
fn add_brut_test() {
    let mut stats = Stats::new_empty();
    stats.set_stat(&BrutaliteSevere, 300);
    let mut stats2 = Stats::new_empty();
    stats2.set_stat(&BrutaliteSevere, 500);
    stats.add_or_remove_brut_stats(&stats2, true);
    assert_eq!(stats.get_stat(&BrutaliteSevere), 800);
}

#[test]
fn full_crap_item_one_amazing_set_test() {
    todo!()
}

#[test]
fn real_item_coherent_result_test(){
    todo!()
}