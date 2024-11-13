use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::builder::utils_mod::utils;

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Breed {
    pub id: i64,
    #[serde(rename = "shortName", deserialize_with = "utils::deserialize_to_string_map")]
    pub names: HashMap<String, String>,
    pub img: String,
    pub img_transparent: String,
}
