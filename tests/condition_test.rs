use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use roxx_builder::builder::build_mod::build::Build;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_condition::ItemCondition;
use roxx_builder::builder::item_mod::item_slot::ItemSlot;

#[test]
fn small_split_test() {
    let s = "My<str str";
    let vec: Vec<&str> = s.splitn(2, |c: char| -> bool { !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) }).collect();
    assert_eq!(vec, vec!["My", "str str"]);
    let s = "My<str strh";
    let vec: Vec<&str> = s.splitn(2, 'h').collect();
    assert_eq!(vec, vec!["My<str str", ""])
}

#[test]
fn no_condition_add_test() {
    let mut build = Build::new();
    assert!(build.add_item(&Item::default(), ItemSlot::SlotAnneau1, false));
}

#[test]
fn same_ring_add_test() {
    let mut build = Build::new();
    let mut ring = Item::default();
    ring.set_id = 1;
    let mut ring2 = Item::default();
    ring2.set_id = 1;
    ring2.id = ring.id;
    assert!(build.add_item(&ring, ItemSlot::SlotAnneau1, false));
    assert_eq!(ring.id, ring2.id);
    assert_eq!(build.add_item(&ring2, ItemSlot::SlotAnneau2, false), false);
}

#[test]
fn same_set_not_same_ring_test() {
    let mut build = Build::new();
    let mut ring = Item::default();
    ring.set_id = 1;
    let mut ring2 = Item::default();
    ring2.set_id = 1;
    assert!(build.add_item(&ring, ItemSlot::SlotAnneau1, false));
    assert_ne!(ring.id, ring2.id);
    assert_eq!(ring.set_id, ring2.set_id);
    assert!(build.add_item(&ring2, ItemSlot::SlotAnneau2, false));
}

fn condition_anneau1_equip_test(condition: ItemCondition, stats: HashMap<BaseStat, i64>, expected: bool) {
    let mut build = Build::new_with(stats);
    let mut item = Item::default();
    item.conditions = condition;
    assert_eq!(build.add_item(&item, ItemSlot::SlotAnneau1, false), expected);
}

#[test]
fn more_stat_than_good_test() {
    let condition = ItemCondition::MoreStatThan(BaseStat::Force, 50);
    condition_anneau1_equip_test(condition, HashMap::from([(BaseStat::Force, 100)]), true)
}

#[test]
fn more_stat_than_fail_test() {
    let condition = ItemCondition::MoreStatThan(BaseStat::Force, 200);
    condition_anneau1_equip_test(condition, HashMap::from([(BaseStat::Force, 100)]), false)
}

#[test]
fn or_condition_good_test() {
    let condition1 = ItemCondition::LessStatThan(BaseStat::Agilite, -1000);
    let condition1_test = ItemCondition::LessStatThan(BaseStat::Agilite, -1000);
    let stats1 = HashMap::from([(BaseStat::Force, 300), (BaseStat::Agilite, -100)]);

    let condition2 = ItemCondition::MoreStatThan(BaseStat::Force, 200);
    let condition2_test = ItemCondition::MoreStatThan(BaseStat::Force, 200);
    let stats2 = HashMap::from([(BaseStat::Force, 300), (BaseStat::Agilite, -100)]);

    let condition = ItemCondition::Or(Box::new(condition1),Box::new( condition2));
    let stats = HashMap::from([(BaseStat::Force, 300), (BaseStat::Agilite, -100)]);

    condition_anneau1_equip_test(condition1_test, stats1, false);
    condition_anneau1_equip_test(condition2_test, stats2, true);
    condition_anneau1_equip_test(condition, stats, true);
}

#[test]
fn or_condition_fail_test() {
    let condition1 = ItemCondition::LessStatThan(BaseStat::Agilite, -1000);
    let condition1_test = ItemCondition::LessStatThan(BaseStat::Agilite, -1000);
    let stats1 = HashMap::from([(BaseStat::Force, 50), (BaseStat::Agilite, -100)]);

    let condition2 = ItemCondition::MoreStatThan(BaseStat::Force, 200);
    let condition2_test = ItemCondition::MoreStatThan(BaseStat::Force, 200);
    let stats2 = HashMap::from([(BaseStat::Force, 50), (BaseStat::Agilite, -100)]);

    let condition = ItemCondition::Or(Box::new(condition1),Box::new( condition2));
    let stats = HashMap::from([(BaseStat::Force, 50), (BaseStat::Agilite, -100)]);

    condition_anneau1_equip_test(condition1_test, stats1, false);
    condition_anneau1_equip_test(condition2_test, stats2, false);
    condition_anneau1_equip_test(condition, stats, false);
}

#[test]
fn and_condition_good_test() {
    let condition1 = ItemCondition::MoreStatThan(BaseStat::Agilite, -1000);
    let condition1_test = ItemCondition::MoreStatThan(BaseStat::Agilite, -1000);
    let stats1 = HashMap::from([(BaseStat::Force, 200), (BaseStat::Agilite, -100)]);

    let condition2 = ItemCondition::StatEqualTo(BaseStat::Force, 200);
    let condition2_test = ItemCondition::StatEqualTo(BaseStat::Force, 200);
    let stats2 = HashMap::from([(BaseStat::Force, 200), (BaseStat::Agilite, -100)]);

    let condition = ItemCondition::And(Box::new(condition1),Box::new( condition2));
    let stats = HashMap::from([(BaseStat::Force, 200), (BaseStat::Agilite, -100)]);

    condition_anneau1_equip_test(condition1_test, stats1, true);
    condition_anneau1_equip_test(condition2_test, stats2, true);
    condition_anneau1_equip_test(condition, stats, true);
}

#[test]
fn and_condition_fail_test() {
    let condition1 = ItemCondition::LessStatThan(BaseStat::Agilite, -1000);
    let condition1_test = ItemCondition::LessStatThan(BaseStat::Agilite, -1000);
    let stats1 = HashMap::from([(BaseStat::Force, 300), (BaseStat::Agilite, -100)]);

    let condition2 = ItemCondition::MoreStatThan(BaseStat::Force, 200);
    let condition2_test = ItemCondition::MoreStatThan(BaseStat::Force, 200);
    let stats2 = HashMap::from([(BaseStat::Force, 300), (BaseStat::Agilite, -100)]);

    let condition = ItemCondition::And(Box::new(condition1),Box::new( condition2));
    let stats = HashMap::from([(BaseStat::Force, 300), (BaseStat::Agilite, -100)]);

    condition_anneau1_equip_test(condition1_test, stats1, false);
    condition_anneau1_equip_test(condition2_test, stats2, true);
    condition_anneau1_equip_test(condition, stats, false);
}

// ==============================================================================================

#[allow(dead_code)]
async fn complete_condition_field() -> Result<(), Box<dyn Error>> { // lol kinda complicated for nothing
    let mut seen_criteria: HashSet<String> = HashSet::new();

    let dir = std::fs::read_dir("tests/test_files/full_api_calls")?;
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let dt = serde_json::from_str(std::fs::read_to_string(path)?.as_str());
            if dt.is_ok() {
                let item_list_json: Value = dt.unwrap();
                let item_list = &item_list_json["data"].as_array();
                if let Some(itm_lst) = item_list {
                    for itm in *itm_lst {
                        if let Some(criteria) = itm["criteria"].as_str() {
                            let split: Vec<&str> = criteria.splitn(2, |c: char| -> bool { !((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) }).collect();
                            seen_criteria.insert(split[0].to_string());
                        }
                    }
                }
            }
        }
    }
    println!("Finished parsed with {} item to call", seen_criteria.len());
    for idk in seen_criteria {
        let answer = reqwest::get(format!("https://api.dofusdb.fr/criterion/{}=1", idk)).await?.text().await?;
        println!("{} | {}", idk, answer);
    }
    Ok(())
}
