use crate::builder::attack_mod::damage_line::DamageLine;
use crate::builder::attack_mod::damage_position::DamagePosition;
use crate::builder::attack_mod::damage_source::DamageSource;

#[allow(dead_code)]
pub struct Attack {
    pub damages: Vec<DamageLine>,
    pub crit_damages: Vec<DamageLine>,
    pub damage_source: DamageSource,
    pub damage_position: DamagePosition,
    pub piege: bool,
    pub can_crit: bool,
    pub base_crit: i32,
}

impl Attack {
    pub fn new(damages: Vec<DamageLine>, crit_damages: Vec<DamageLine>, damage_source: DamageSource, damage_position: DamagePosition, can_crit: bool, base_crit: i32) -> Self {
        Attack {
            damages,
            crit_damages,
            damage_source,
            damage_position,
            piege: false,
            can_crit,
            base_crit
        }
    }

    pub fn default() -> Self{
        Attack{
            damages: vec![],
            crit_damages: vec![],
            damage_source: DamageSource::Sort,
            damage_position: DamagePosition::Distance,
            piege: false,
            can_crit: false,
            base_crit: 0
        }
    }
}