use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum DamageCalculation {
    Minimized,
    Min,
    Average,
    Max,
}