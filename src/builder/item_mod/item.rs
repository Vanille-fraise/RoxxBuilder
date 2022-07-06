use std::collections::HashMap;
use rand::random;
use crate::builder::item_mod::item_type::ItemType;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use serde::{Deserialize, Serialize};
use crate::builder::item_mod::item_condition::ItemCondition;
use crate::builder::item_mod::item_type::ItemType::Amulette;

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct Item {
    pub item_type: ItemType,
    pub stats: HashMap<BaseStat, i32>,
    pub name: String,
    pub lvl: u32,
    pub set_id: usize,
    pub conditions: Vec<ItemCondition>,
    pub id: usize,
}

#[allow(dead_code)]
impl Item {
    pub fn from_str(_str: String) -> Item { todo!() }

    pub fn default() -> Self {
        Item {
            item_type: Amulette,
            stats: HashMap::new(),
            name: "No name".to_string(),
            lvl: 200,
            set_id: 0,
            conditions: vec![],
            id: random(),
        }
    }
    pub fn new_with_stats(stats: HashMap<BaseStat, i32>) -> Self {
        let mut item = Item::default();
        item.stats = stats;
        item
    }
    pub fn new(item_type: ItemType, stats: HashMap<BaseStat, i32>, name: String, lvl: u32, set_id: usize, conditions: Vec<ItemCondition>, item_id: usize) -> Self {
        Item{
            item_type,
            stats,
            name,
            lvl,
            set_id,
            conditions,
            id: item_id,
        }
    }
    pub fn new_from_type(item_type: ItemType) -> Self {
        let mut item = Item::default();
        item.item_type = item_type;
        item
    }
}