use crate::builder::attack_mod::damage_line::DamageLine;
use crate::builder::attack_mod::damage_position::DamagePosition;
use crate::builder::attack_mod::damage_source::DamageSource;
use serde::{Serialize, Deserialize};
use crate::builder::attack_mod::damage_calculation::DamageCalculation;
use crate::builder::attack_mod::damage_element::DamageElement;

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
    brutality_crit_damage: i64,
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
            brutality_crit_damage: -1,
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
            brutality_crit_damage: 0,
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
        self.brutality_damage = match self.damage_calculation {
            DamageCalculation::Minimized | DamageCalculation::Min => { self.damages.iter().map(|l| { l.min_value }).sum() }
            DamageCalculation::Average => { self.damages.iter().map(|l| { (l.min_value + l.max_value) / 2 }).sum() }
            DamageCalculation::Max => { self.damages.iter().map(|l| { l.max_value }).sum() }
        };
        self.brutality_crit_damage = match self.damage_calculation {
            DamageCalculation::Minimized | DamageCalculation::Min => { self.crit_damages.iter().map(|l| { l.min_value }).sum() }
            DamageCalculation::Average => { self.crit_damages.iter().map(|l| { (l.min_value + l.max_value) / 2 }).sum() }
            DamageCalculation::Max => { self.crit_damages.iter().map(|l| { l.max_value }).sum() }
        };
    }
    pub fn brutality_crit_damage(&self) -> i64 {
        self.brutality_crit_damage
    }

    pub fn get_every_damage_lines(&self) -> Vec<(&DamageElement, i64, bool)> {
        let mut result = vec![];
        let all_lines = [(&self.damages, false), (&self.crit_damages, true)];
        for (lines, is_crit) in all_lines {
            let mut cur_res: Vec<(&DamageElement, i64, bool)> = match self.damage_calculation() {
                DamageCalculation::Minimized => { lines.iter().map(|l| { (&l.damage_element, l.min_value, is_crit) }).collect() }
                DamageCalculation::Min => { lines.iter().map(|l| { (&l.damage_element, l.min_value, is_crit) }).collect() }
                DamageCalculation::Average => { lines.iter().map(|l| { (&l.damage_element, (l.min_value + l.max_value) / 2, is_crit) }).collect() }
                DamageCalculation::Max => { lines.iter().map(|l| { (&l.damage_element, l.max_value, is_crit) }).collect() }
            };
            result.append(&mut cur_res);
        }
        result
    }
}