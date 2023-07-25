use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum DamageSource {
    Sort,
    Arme,
}

impl DamageSource {
    pub fn default() -> Self {
        DamageSource::Sort
    }
}