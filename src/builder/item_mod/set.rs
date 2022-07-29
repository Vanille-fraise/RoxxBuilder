use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Set {
    pub id: i64,
    pub bonus: Vec<HashMap<BaseStat, i64>>,
    name: String,
}

impl Set {
    pub fn new(id: i64, bonus: Vec<HashMap<BaseStat, i64>>) -> Self {
        Set {
            id,
            bonus,
            name: "No name".to_string(),
        }
    }

    pub fn from_serde_value(values: &serde_json::Value) -> Self {
        Set {
            id: values["id"].as_i64().unwrap_or(-1),
            bonus: values["effects"].as_array().unwrap_or(&vec![]).iter().map(BaseStat::from_effects_json_value).collect(),
            name: values["name"].as_str().unwrap_or("No name found").to_string(),
        }
    }
}