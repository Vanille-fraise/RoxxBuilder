use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Set {
    pub id: i64,
    pub bonus: Vec<HashMap<BaseStat, i64>>,
}

impl Set {
    pub fn new(id: i64, bonus: Vec<HashMap<BaseStat, i64>>) -> Self {
        Set {
            id,
            bonus,
        }
    }
}