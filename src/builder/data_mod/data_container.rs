use crate::builder::attack_mod::attack::Attack;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::set::Set;
use serde::{Serialize, Deserialize};

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
            attacks: vec![]
        }
    }
}