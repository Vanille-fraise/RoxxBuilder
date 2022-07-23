use std::collections::HashMap;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::set::Set;
use serde::{Serialize, Deserialize};
use crate::builder::item_mod::item_type::ItemType;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataContainer<'a> {
    pub items: Vec<Item<'a>>,
    pub sets: Vec<Set>,
    pub attacks: Vec<Attack>,
}

impl<'a> DataContainer<'a> {
    pub fn new() -> Self {
        DataContainer {
            items: vec![],
            sets: vec![],
            attacks: vec![],
        }
    }

    pub fn clear_unknown_type(&mut self) {
        let mut i = 0;
        while i < self.items.len() {
            if self.items[i].item_type == ItemType::Unknown {
                self.items.remove(i);
            } else { i += 1; }
        }
    }

    pub fn link_item_with_set(&'a mut self) {
        let mut map: HashMap<i64, &'a Set> = HashMap::default();
        for set in &self.sets {
            map.insert(set.id, set);
        }
        for item in self.items.iter_mut() {
            if item.set_id > 0 && map.get(&item.set_id).is_some() {
                let cur_set = map.get(&item.set_id).unwrap();
                item.set = Some(cur_set);
            }
        }
    }
}