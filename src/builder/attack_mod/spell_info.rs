use std::{collections::HashMap, sync::Arc};
use serde::{Deserialize, Serialize};
use crate::builder::utils_mod::utils;

use super::attack::Attack;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellInfo {
    id: i64,
    #[serde(rename = "spellLevels")]
    pub spell_levels_id: Vec<i64>,
    #[serde(alias = "name", deserialize_with = "utils::deserialize_to_string_map")]
    pub names: HashMap<String, String>,
    pub img: String,
    #[serde(default)]
    pub attacks: Vec<Arc<Attack>>,
}
