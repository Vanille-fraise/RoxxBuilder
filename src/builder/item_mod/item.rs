use std::collections::HashMap;
use rand::random;
use crate::builder::item_mod::item_type::ItemType;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use serde::{Deserialize, Serialize};
use crate::builder::item_mod::item_condition::ItemCondition;
use crate::builder::item_mod::item_type::ItemType::Amulette;
use crate::builder::item_mod::set::Set;
extern crate serde;
extern crate serde_json;

extern crate num;


#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Item<'a> {
    pub item_type: ItemType,
    pub stats: HashMap<BaseStat, i64>,
    pub name: String,
    pub lvl: u64,
    pub set_id: i64,
    #[serde(skip_deserializing)]
    #[serde(default)]
    pub set: Option<&'a Set>,
    pub conditions: ItemCondition,
    pub id: i64,
}

impl<'a> Item<'a> {
    pub fn from_serde_value(values: &serde_json::Value) -> Self {
        Item {
            item_type: num::FromPrimitive::from_u64(values["typeId"].as_u64().unwrap_or(0)).unwrap_or(ItemType::Unknown),
            stats: BaseStat::from_effects_json_value(&values["effects"]), // todo: add stats
            name: values["name"]["fr"].as_str().unwrap_or("No Name").to_string(),
            lvl: values["level"].as_u64().unwrap_or(1),
            set_id: values["itemSetId"].as_i64().unwrap_or(-1),
            set: None,
            conditions: ItemCondition::from_dofus_db_str(values["criteria"].as_str().unwrap_or("")),
            id: values["id"].as_i64().unwrap_or(0),
        }
    }
    pub fn default() -> Self {
        Item {
            item_type: Amulette,
            stats: HashMap::new(),
            name: "No name".to_string(),
            lvl: 200,
            set_id: -1,
            set: None,
            conditions: ItemCondition::None,
            id: random(),
        }
    }
    pub fn new_with_stats(stats: HashMap<BaseStat, i64>) -> Self {
        let mut item = Item::default();
        item.stats = stats;
        item
    }
    pub fn new(item_type: ItemType, stats: HashMap<BaseStat, i64>, name: String, lvl: u64, set_id: i64, conditions: ItemCondition, item_id: i64, set: Option<&'a Set>) -> Self {
        Item {
            item_type,
            stats,
            name,
            lvl,
            set_id,
            set,
            conditions,
            id: item_id,
        }
    }
    pub fn new_from_type(item_type: ItemType) -> Self {
        let mut item = Item::default();
        item.item_type = item_type;
        item
    }

    pub fn empty() ->  Self {
        let mut item = Self::default();
        item.name = "No item".to_string();
        item
    }
}


#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
struct IntermediaryStruct {
    id: i64,
    item_type: isize,
}