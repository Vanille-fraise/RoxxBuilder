use serde::{Serialize, Deserialize};
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;

#[repr(usize)]
#[derive(Serialize, Deserialize, Debug, EnumIter, Clone)]
pub enum DamageElement {
    DamageTerre = 0,
    DamageEau,
    DamageFeu,
    DamageNeutre,
    DamageAir,
}

impl DamageElement {
    pub fn stat_based_on(&self) -> BaseStat {
        match self {
            DamageElement::DamageTerre | DamageElement::DamageNeutre => { BaseStat::Force }
            DamageElement::DamageEau => { BaseStat::Chance }
            DamageElement::DamageFeu => { BaseStat::Intelligence }
            DamageElement::DamageAir => { BaseStat::Agilite }
        }
    }
    pub fn damage_based_on(&self) -> BaseStat {
        match self {
            DamageElement::DamageTerre => { BaseStat::DoTerre }
            DamageElement::DamageEau => { BaseStat::DoEau }
            DamageElement::DamageFeu => { BaseStat::DoFeu }
            DamageElement::DamageAir => { BaseStat::DoAir }
            DamageElement::DamageNeutre => { BaseStat::DoNeutre }
        }
    }
}