use serde::{Deserialize, Serialize};
use super::spell_info::SpellInfo;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpellVariant {
    pub id: i64, 
    pub spell_ids: Vec<i64>,
    pub breed_id: i64,
    pub spells: Vec<SpellInfo>,
}
