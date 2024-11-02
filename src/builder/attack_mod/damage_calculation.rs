use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum DamageCalculation {
    Minimized,
    Min,
    Average,
    Max,
}

impl Default for DamageCalculation {
    fn default() -> Self {
        DamageCalculation::Average
    }
}