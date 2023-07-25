use serde::{Serialize, Deserialize};
use crate::builder::attack_mod::damage_position::DamagePosition::Distance;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum DamagePosition {
    Distance,
    Melee,
}

impl DamagePosition {
    pub fn default() -> Self {
        Distance
    }
}