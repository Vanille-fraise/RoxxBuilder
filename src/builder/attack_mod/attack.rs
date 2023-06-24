use crate::builder::attack_mod::damage_line::DamageLine;
use crate::builder::attack_mod::damage_position::DamagePosition;
use crate::builder::attack_mod::damage_source::DamageSource;
use serde::{Serialize, Deserialize};
use crate::builder::attack_mod::damage_calculation::DamageCalculation;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    damages: Vec<DamageLine>,
    crit_damages: Vec<DamageLine>,
    pub damage_source: DamageSource,
    pub damage_position: DamagePosition,
    pub piege: bool,
    pub can_crit: bool,
    pub base_crit: i64,
    brutality_damage: i64,
    damage_calculation: DamageCalculation,
}

impl Attack {
    pub fn new(damages: Vec<DamageLine>, crit_damages: Vec<DamageLine>, damage_source: DamageSource, damage_position: DamagePosition, can_crit: bool, base_crit: i64, damage_calculation: DamageCalculation) -> Self {
        let mut attack = Attack {
            damages,
            crit_damages,
            damage_source,
            damage_position,
            piege: false,
            can_crit,
            base_crit,
            brutality_damage: -1,
            damage_calculation,
        };
        attack.compute_brutality_damage();
        attack
    }

    pub fn default() -> Self {
        Attack {
            damages: vec![],
            crit_damages: vec![],
            damage_source: DamageSource::Sort,
            damage_position: DamagePosition::Distance,
            piege: false,
            can_crit: false,
            base_crit: 0,
            brutality_damage: 0,
            damage_calculation: DamageCalculation::Average,
        }
    }

    pub fn brutality_damage(&self) -> i64 {
        self.brutality_damage
    }

    pub fn damages(&self) -> &Vec<DamageLine> {
        &self.damages
    }

    pub fn crit_damages(&self) -> &Vec<DamageLine> {
        &self.crit_damages
    }

    pub fn set_damages(&mut self, damages: Vec<DamageLine>) {
        self.damages = damages;
        self.compute_brutality_damage();
    }

    pub fn set_crit_damages(&mut self, crit_damages: Vec<DamageLine>) {
        self.crit_damages = crit_damages;
        self.compute_brutality_damage();
    }

    pub fn damage_calculation(&self) -> DamageCalculation {
        self.damage_calculation
    }

    pub fn set_damage_calculation(&mut self, damage_calculation: DamageCalculation) {
        self.damage_calculation = damage_calculation;
        self.compute_brutality_damage();
    }

    pub fn compute_brutality_damage(&mut self) {
        let all_lines = self.damages.iter().chain(&self.crit_damages);
        self.brutality_damage = match self.damage_calculation {
            DamageCalculation::Minimized => {all_lines.map(|l| { l.min_value }).sum() }
            DamageCalculation::Min => { all_lines.map(|l| { l.min_value }).sum() }
            DamageCalculation::Average => { all_lines.map(|l| { (l.min_value + l.max_value) / 2 }).sum() }
            DamageCalculation::Max => { all_lines.map(|l| { l.max_value }).sum() }
        };
    }
}