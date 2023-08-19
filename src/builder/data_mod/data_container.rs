use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::set::Set;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::builder::item_mod::item_type::ItemType;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataContainer {
    pub items: Vec<Item>,
    pub sets: Vec<Arc<Set>>,
    pub attacks: Vec<Attack>,
}

impl DataContainer {
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

    pub fn link_item_with_set(&mut self) {
        let mut map: HashMap<i64, Arc<Set>> = HashMap::default();
        for set in self.sets.iter() {
            map.insert(set.id, set.clone());
        }
        for item in self.items.iter_mut() {
            if item.set_id > 0 && map.get(&item.set_id).is_some() {
                let cur_set = map.get(&item.set_id).unwrap();
                item.set = Some(cur_set.clone());
            }
        }
    }
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn add_item_from_value(&mut self, value: Value) {
        let item = Item::from_serde_value(&value);
        self.items.push(item);
    }

    pub fn add_set(&mut self, set: Set) {
        self.sets.push(Arc::new(set));
    }

    pub fn reset_brutality(&mut self, attack: &Attack) {
        for item in self.items.iter_mut() {
            item.stats.reset_brutality(attack);
        }
        let mut new_sets: Vec<Arc<Set>> = vec![];
        for set in self.sets.iter() {
            let mut cur_set = set.deref().clone();
            for bonus in cur_set.bonus.iter_mut() {
                bonus.reset_brutality(attack);
            }
            new_sets.push(Arc::new(cur_set));
        }
        self.sets = new_sets;
        self.link_item_with_set();
    }

    pub fn get_items_with_ids(&self, ids: &Vec<i64>) -> Vec<&Item> {
        let set_ids: Vec<i64> = self.sets.iter().filter(|s| ids.contains(&s.id)).map(|s| s.id).collect();
        self.items.iter().filter(|&i| set_ids.contains(&i.set.clone().unwrap().id) || ids.contains(&i.id)).collect()
    }
}