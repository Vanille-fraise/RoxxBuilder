use serde::Deserialize;
use crate::builder::attack_mod::attack::Attack;

#[derive(Deserialize, Clone)]
pub struct PostAttackData {
    #[serde(flatten)]
    attack: Attack,
}

impl PostAttackData {
    pub fn attack(&self) -> &Attack {
        &self.attack
    }
}