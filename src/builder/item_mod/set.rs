use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;

#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    pub id: i64,
    pub bonus: Vec<HashMap<BaseStat, i64>>,
}