use std::collections::HashMap;
use std::time::Instant;
use itertools::Itertools;
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
use roxx_builder::builder::pplayer_mod::player::SearchOptions;
use roxx_builder::builder::data_mod::data_container::DataContainer;
use roxx_builder::builder::data_mod::data_manager::DataManager;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::{BrutaliteSevere, Force};
use roxx_builder::builder::item_mod::item_slot::ItemSlot;
use roxx_builder::builder::item_mod::item_type::ItemType::{Amulette, Anneau, Arc, Bottes, Bouclier, Cape, Ceinture, Chapeau, Dofus, Familier, Prysmaradite, Trophee};
use roxx_builder::builder::item_mod::set::Set;
use roxx_builder::builder::item_mod::stats::Stats;

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref COCO : Attack = Attack::new(vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], vec![DamageLine::new_fix(DamageElement::DamageTerre, 100)], DamageSource::Sort, DamagePosition::Melee, true, 25, DamageCalculation::Average);
}
lazy_static! {
    static ref REAL_COCO : Attack = Attack::new(vec![DamageLine::new(DamageElement::DamageTerre, 81, 100)], vec![DamageLine::new(DamageElement::DamageTerre, 97, 120)], DamageSource::Sort, DamagePosition::Melee, true, 25, DamageCalculation::Max);
}
lazy_static! {
    static ref TYPES: Vec<ItemType> = vec![Amulette, Arc, Ceinture, Bottes, Chapeau, Cape, Familier, Prysmaradite, Trophee, Dofus, Dofus, Trophee, Dofus, Bouclier, Anneau];
}
lazy_static! {
    static ref BLACK_LIST: Vec<String> = vec!["Surpuissant Chacha de Combat (MJ)".to_string(), "Annobusé de Maître Jarbo".to_string(), "Petit Chacha de Combat (MJ)".to_string()];
}


#[test]
fn empty_test() {
    let dc = DataContainer::new();
    let res = GraphLooker::search(&dc, &COCO, &SearchOptions::empty());
    assert_eq!(res.builds_evaluated, 0);
    assert_eq!(res.build.stats, Stats::empty());
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
    let mut dc = DataContainer::new();
    for t in TYPES.iter() {
        let mut item = Item::new_with_stats(Stats::from_map_stats(HashMap::from([(BaseStat::DoMulti, 10), (BaseStat::Puissance, 100)]).iter()));
        item.item_type = t.clone();
        item.name = "Best".to_string();
        dc.items.push(item);
    }
    // testing
    // dc.items.pop();
    for i in 0..5000usize {
        let mut item = Item::new_with_stats(Stats::from_map_stats(HashMap::from([(BaseStat::DoMulti, (i % 3) as i64), (BaseStat::Puissance, (i % 5) as i64)]).iter()));
        item.item_type = TYPES.get(i % TYPES.len()).unwrap().clone();
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
    let mut stats = Stats::empty();
    stats.set_stat(&BrutaliteSevere, 300);
    let mut stats2 = Stats::empty();
    stats2.set_stat(&BrutaliteSevere, 500);
    stats.add_or_remove_brut_stats(&stats2, true);
    assert_eq!(stats.get_stat(&BrutaliteSevere), 800);
}

#[test]
fn full_crap_item_one_amazing_set_test() {
    let mut dc = DataContainer::new();
    let set_id = 17;
    let mut item = Item::default();
    let mut item2 = Item::default();
    item2.item_type = Bottes;
    let set = Set::new(set_id, vec![Stats::empty(),
                                    Stats::from_map_stats(HashMap::from([(BaseStat::DoMulti, 80), (BaseStat::Puissance, 350)]).iter()),
                                    Stats::from_map_stats(HashMap::from([(BaseStat::DoMulti, 250), (BaseStat::Puissance, 800)]).iter())]);
    item.set_id = set_id;
    item2.set_id = set_id;

    dc.items.push(item);
    dc.items.push(item2);
    dc.sets.push(set.into());
    dc.link_sub_data();

    for i in 0..5000usize {
        let mut item = Item::new_with_stats(Stats::from_map_stats(HashMap::from([(BaseStat::DoMulti, (i % 27) as i64), (BaseStat::Puissance, (i % 117) as i64)]).iter()));
        item.item_type = TYPES.get(i % TYPES.len()).unwrap().clone();
        dc.items.push(item);
    }
    let mut search_option = SearchOptions::empty();
    search_option.set_search_time_milli(3000);
    let res = GraphLooker::search(&dc, &COCO, &search_option);
    println!("Build evaluated: {}", res.builds_evaluated);
    println!("Build per sec: {}", res.builds_evaluated / 3);
    println!("Eval: {}", res.eval);
    println!("Eval pos: {}", res.best_build_position);
    println!("Addition: {:?}", res.additional_data);
    assert!(res.eval > 2500);
}


/**
 * Clone of test function in estimator_and_prio_test
 **/
pub fn generate_test_dc() -> DataContainer {
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
    dc.link_sub_data();
    dc
}

#[test]
fn full_crap_find_right_comb_graph_version() {
    let dc = generate_test_dc();
    let mut search_option = SearchOptions::empty();
    search_option.set_search_time_milli(10_000);
    search_option.set_black_list(BLACK_LIST.clone());
    let res = GraphLooker::search(&dc, &COCO, &search_option);
    println!("Build evaluated: {}", res.builds_evaluated);
    assert_eq!(res.eval, 1860);
}

#[actix_rt::test]
async fn real_item_coherent_result_test() {
    let time = Instant::now();
    let mut dc = DataManager::retrieve_data().await;
    dc.clear_unknown_type_items();
    println!("Data container loaded in: {}ms", time.elapsed().as_millis());
    let mut rng = rand::thread_rng();
    dc.items.shuffle(&mut rng);
    dc.link_sub_data();
    let mut search_option = SearchOptions::empty();
    search_option.set_search_time_milli(10_000);
    search_option.set_black_list(BLACK_LIST.clone());
    let mut res = GraphLooker::search(&dc, &REAL_COCO, &search_option);
    println!("Build evaluated: {}", res.builds_evaluated);
    println!("Build per sec: {}", res.builds_evaluated / (search_option.search_time_milli() / 1000) as i64);
    println!("Builds considered: {}", res.builds_considered);
    res.build.stats.reset_brutality(&REAL_COCO);
    println!("Eval: {}", res.eval);
    println!("Build eval: {}", res.build.evaluate_build_damage(&REAL_COCO));
    println!("Eval pos: {}", res.best_build_position);
    println!("Addition: {:?}", res.additional_data);
    println!("{}", res.build.to_string());
    assert_eq!(res.build.items.iter().map(|i| ItemSlot::corresponding_to_item_type(&i.item_type)).flatten().unique().count(), 16);
    assert!(res.eval > 2500);
    // todo il met 6 fois le meme trophée
}

/*
#[actix_rt::test]
async fn best_set_than_item_but_not_cumul_bonus_test() {
    todo!()
}
*/