use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use strum_macros::{EnumIter, EnumString};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[repr(i64)]
#[derive(Debug, EnumIter, Clone, EnumString, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[serde(rename_all = "camelCase")]
pub enum DamageElement {
    // Be careful with None, it should be used to handle heal or push damage. but i shouldn't have damage line with no damage.
    None = -1,
    DamageNeutre = 0,
    DamageTerre = 1,
    DamageFeu = 2,
    DamageEau = 3,
    DamageAir = 4,
    Variable = 5,
}

impl DamageElement {
    pub fn stat_based_on(&self) -> BaseStat {
        match self {
            DamageElement::DamageTerre | DamageElement::DamageNeutre => BaseStat::Force,
            DamageElement::DamageEau => BaseStat::Chance,
            DamageElement::DamageFeu => BaseStat::Intelligence,
            DamageElement::DamageAir => BaseStat::Agilite,
            DamageElement::None | DamageElement::Variable => BaseStat::Unknown,
        }
    }
    pub fn damage_based_on(&self) -> BaseStat {
        match self {
            DamageElement::DamageTerre => BaseStat::DoTerre,
            DamageElement::DamageEau => BaseStat::DoEau,
            DamageElement::DamageFeu => BaseStat::DoFeu,
            DamageElement::DamageAir => BaseStat::DoAir,
            DamageElement::DamageNeutre => BaseStat::DoNeutre,
            DamageElement::None | DamageElement::Variable => BaseStat::Unknown,
        }
    }

    pub fn default() -> Self {
        DamageElement::DamageNeutre
    }
}
