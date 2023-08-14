use rand;
use rand::prelude::*;
use rand::Rng; // 0.8.5

use crate::builder::item_mod::item_type::ItemType;
use serde::{Deserialize, Serialize};
use crate::builder::item_mod::item_condition::ItemCondition;
use crate::builder::item_mod::item_type::ItemType::Amulette;
use crate::builder::item_mod::set::Set;
use crate::builder::item_mod::stats::Stats;


extern crate serde;
extern crate serde_json;

extern crate num;

pub static EMPTY_ITEMS: [Item; 16] = [
    Item { item_type: Amulette, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 0 },
    Item { item_type: ItemType::Bottes, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 1 },
    Item { item_type: ItemType::Bouclier, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 2 },
    Item { item_type: ItemType::Cape, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 3 },
    Item { item_type: ItemType::Arc, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 4 },
    Item { item_type: ItemType::Familier, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 5 },
    Item { item_type: ItemType::Ceinture, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 6 },
    Item { item_type: ItemType::Chapeau, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 7 },
    Item { item_type: ItemType::Anneau, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 8 },
    Item { item_type: ItemType::Anneau, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 9 },
    Item { item_type: ItemType::Dofus, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 10 },
    Item { item_type: ItemType::Dofus, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 11 },
    Item { item_type: ItemType::Dofus, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 12 },
    Item { item_type: ItemType::Dofus, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 13 },
    Item { item_type: ItemType::Dofus, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 14 },
    Item { item_type: ItemType::Prysmaradite, stats: Stats::new_empty(), name: String::new(), lvl: 1, set_id: 0, set: None, conditions: ItemCondition::None, id: 15 }
];

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Item<'a> {
    pub item_type: ItemType,
    pub stats: Stats,
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
            stats: Stats::from_effects_json_value(&values["effects"]),
            name: String::from(values["name"]["fr"].as_str().unwrap_or("No Name")),
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
            stats: Stats::new_empty(),
            name: String::from("No name"),
            lvl: 200,
            set_id: -1,
            set: None,
            conditions: ItemCondition::None,
            id: thread_rng().gen_range(17..i64::MAX),
        }
    }
    pub fn new_with_stats(stats: Stats) -> Self {
        let mut item = Item::default();
        item.stats = stats;
        item
    }
    pub fn new(item_type: ItemType, stats: Stats, name: String, lvl: u64, set_id: i64, conditions: ItemCondition, item_id: i64, set: Option<&'a Set>) -> Self {
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

    pub fn empty() -> Self {
        let mut item = Self::default();
        item.name = String::from("No item");
        item
    }

    pub fn ref_empty_items() -> [&'static Item<'static>; 16] {
        [
            &EMPTY_ITEMS[0], &EMPTY_ITEMS[1], &EMPTY_ITEMS[2], &EMPTY_ITEMS[3],
            &EMPTY_ITEMS[4], &EMPTY_ITEMS[5], &EMPTY_ITEMS[6], &EMPTY_ITEMS[7],
            &EMPTY_ITEMS[8], &EMPTY_ITEMS[9], &EMPTY_ITEMS[10], &EMPTY_ITEMS[11],
            &EMPTY_ITEMS[12], &EMPTY_ITEMS[13], &EMPTY_ITEMS[14], &EMPTY_ITEMS[15],
        ]
    }
}

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
struct IntermediaryStruct {
    id: i64,
    item_type: isize,
}