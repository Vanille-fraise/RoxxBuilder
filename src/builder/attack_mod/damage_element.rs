use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DamageElement {
    DamageTerre,
    DamageEau,
    DamageFeu,
    DamageNeutre,
    DamageAir,
}