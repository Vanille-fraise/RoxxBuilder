use crate::builder::{
    build_mod::build_search_result::BuildSearchResult,
    item_mod::{item::Item, item_slot::ItemSlot, item_type::ItemType},
};
use serde::Serialize;
use std::{collections::HashMap, time::Duration};
use num::FromPrimitive;

#[derive(Serialize)]
pub struct ResponseItemInfo {
    pub item_type: ItemType,
    pub name: String,
    pub names: HashMap<String, String>,
    pub lvl: u64,
    pub set_id: i64,
    pub id: i64,
    pub img: String,
    pub imgs: HashMap<i64, String>,
}

impl ResponseItemInfo {
    pub fn from_item(item: &Item) -> Self {
        ResponseItemInfo {
            item_type: item.item_type,
            name: item.name.clone(),
            names: item.names.as_ref().unwrap_or(&HashMap::new()).clone(),
            lvl: item.lvl,
            set_id: item.set_id,
            id: item.id,
            img: item.img.clone(),
            imgs: item.imgs.as_ref().unwrap_or(&HashMap::new()).clone(),
        }
    }
}

#[derive(Serialize)]
pub struct ResponseBuildData {
    items_name: [String; 16],
    eval: i64,
    builds_evaluated: i64,
    search_time: Duration,
    best_build_position: i64,
    items: HashMap<ItemSlot,ResponseItemInfo>,
}

impl ResponseBuildData {
    pub fn new(items_name: [String; 16], eval: i64, builds_evaluated: i64, search_time: Duration, best_build_position: i64, items: HashMap<ItemSlot,ResponseItemInfo>,) -> Self {
        ResponseBuildData {
            items_name,
            eval,
            builds_evaluated,
            search_time,
            best_build_position,
            items,
        }
    }

    pub fn from_build_search_result(search_result: &BuildSearchResult) -> Self {
        Self::new(
            search_result.build.items.map(|item| item.name.clone()),
            search_result.eval,
            search_result.builds_evaluated,
            search_result.search_time,
            search_result.best_build_position,
            search_result.build.items.iter().enumerate().map(|(item_slot, item)| (ItemSlot::from_usize(item_slot).unwrap(), ResponseItemInfo::from_item(item))).collect()
        )
    }
}
