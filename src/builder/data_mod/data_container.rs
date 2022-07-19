use crate::builder::attack_mod::attack::Attack;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::set::Set;
use serde::{Serialize, Deserialize};
use crate::builder::item_mod::item_type::ItemType;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataContainer {
    pub items: Vec<Item>,
    pub sets: Vec<Set>,
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
}