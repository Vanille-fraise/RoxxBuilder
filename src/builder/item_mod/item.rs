use std::collections::HashMap;
use rand::random;
use crate::builder::item_mod::item_type::ItemType;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use serde::{Deserialize, Serialize};
use crate::builder::item_mod::item_condition::ItemCondition;
use crate::builder::item_mod::item_type::ItemType::Amulette;
extern crate num;


#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Item {
    pub item_type: ItemType,
    pub stats: HashMap<BaseStat, i64>,
    pub name: String,
    pub lvl: u64,
    pub set_id: i64,
    pub conditions: Vec<ItemCondition>,
    pub id: u64,
}

impl Item {
    pub fn from_serde_value(values: serde_json::Value) -> Self {
        Item {
            item_type: num::FromPrimitive::from_u64(values["typeId"].as_u64().unwrap_or(0)).unwrap(),
            stats: Default::default(),
            name: values["name"]["fr"].as_str().unwrap_or("No Name").to_string(),
            lvl: values["level"].as_u64().unwrap_or(1),
            set_id: values["itemSetId"].as_i64().unwrap_or(-1),
            conditions: vec![],
            id: values["id"].as_u64().unwrap_or(0)
        }
    }
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
    pub fn new_with_stats(stats: HashMap<BaseStat, i64>) -> Self {
        let mut item = Item::default();
        item.stats = stats;
        item
    }
    pub fn new(item_type: ItemType, stats: HashMap<BaseStat, i64>, name: String, lvl: u64, set_id: i64, conditions: Vec<ItemCondition>, item_id: u64) -> Self {
        Item {
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


#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
struct IntermediaryStruct {
    id: i64,
    item_type: isize,
}