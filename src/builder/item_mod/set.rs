use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use crate::builder::item_mod::stats::Stats;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub id: i64,
    pub bonus: Vec<Stats>,
    name: String,
}

impl Set {
    pub fn new(id: i64, bonus: Vec<Stats>) -> Self {
        Set {
            id,
            bonus,
            name: "No name".to_string(),
        }
    }

    pub fn from_serde_value(values: &serde_json::Value) -> Self {
        Set {
            id: values["id"].as_i64().unwrap_or(-1),
            bonus: values["effects"].as_array().unwrap_or(&vec![]).iter().map(Stats::from_effects_json_value).collect(),
            name: values["name"].as_str().unwrap_or("No name found").to_string(),
        }
    }
}

impl PartialOrd<Self> for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Set {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
