use serde::Deserialize;
use crate::builder::attack_mod::attack::Attack;

#[derive(Deserialize, Clone)]
pub struct PostAttackData {
    #[serde(flatten)]
    pub attack: Attack,
}
