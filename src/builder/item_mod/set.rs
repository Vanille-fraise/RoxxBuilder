use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;

#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    id:i64,
    bonus: Vec<HashMap<BaseStat, i64>>
}