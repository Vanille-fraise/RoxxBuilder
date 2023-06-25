use std::cmp::max;
use serde::{Deserialize, Serialize};

extern crate serde;

use serde_big_array::BigArray;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::attack_mod::damage_position::DamagePosition;
use crate::builder::attack_mod::damage_source::DamageSource;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat::{DoPerArme, DoPerDist, DoPerMelee, DoPerSo};

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
pub struct Stats {
    #[serde(with = "BigArray")]
    base_stats: [i64; 53],
    brutality_stats: [i64; 6],
}

impl Stats {
    pub const fn new_empty() -> Self {
        Stats {
            base_stats: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            brutality_stats: [0, 0, 0, 0, 0, 0],
        }
    }
    pub fn from_effects_json_value(value: &serde_json::Value) -> Self {
        let mut stats = Stats::new_empty();
        value.as_array().unwrap_or(&mut vec![]).iter().filter(|v| { v["characteristic"].as_i64().unwrap_or(-1) > 0 }).for_each(|v| {
            let stat = BaseStat::from_dofus_db_val(v["characteristic"].as_i64().unwrap());
            let to = v["to"].as_i64().unwrap_or(0);
            let from = v["from"].as_i64().unwrap_or(0);
            stats.set_stat(&stat, max(to, from));
        });
        stats
    }

    pub fn get_stat(&self, stat: &BaseStat) -> i64 {
        // /!\ Not safe: if out of bound crashes
        self.get_stat_pos(*stat as usize)
    }

    pub fn get_stat_pos(&self, stat_pos: usize) -> i64 {
        // /!\ Not safe: if out of bound crashes
        if stat_pos < self.base_stats.len() {
            self.base_stats[stat_pos]
        } else {
            self.brutality_stats[stat_pos % self.base_stats.len()]
        }
    }
    pub fn set_stat(&mut self, stat: &BaseStat, value: i64) -> bool {
        let stat_pos = *stat as usize;
        self.set_stat_pos(stat_pos, value)
    }
    pub fn set_stat_pos(&mut self, stat_pos: usize, value: i64) -> bool {
        if stat_pos < self.base_stats.len() {
            self.base_stats[stat_pos] = value;
            true
        } else if stat_pos - self.base_stats.len() < self.brutality_stats.len() {
            self.brutality_stats[stat_pos - self.base_stats.len()] = value;
            true
        } else { false }
    }

    pub fn add_or_remove_stats(&mut self, other_stats: &Stats, add: bool) {
        for i in 0..(self.base_stats.len() + self.brutality_stats.len()) {
            self.set_stat_pos(i, self.get_stat_pos(i) + other_stats.get_stat_pos(i) * if add { 1 } else { -1 });
        }
    }

    pub fn add_or_remove_brut_stats(&mut self, other_stats: &Stats, add: bool) {
        if add {
            for i in 0..self.brutality_stats.len() {
                self.brutality_stats[i] += other_stats.base_stats[i];
            }
        } else {
            for i in 0..self.brutality_stats.len() {
                self.brutality_stats[i] -= other_stats.base_stats[i];
            }
        }
    }

    pub fn from_map_stats<'a>(stats_map: impl Iterator<Item=(&'a BaseStat, &'a i64)>) -> Self
    {
        let mut stats: Self = Self::new_empty();
        for (stat, val) in stats_map {
            stats.set_stat(&stat.clone(), *val);
        }
        return stats;
    }

    pub fn reset_brutality(&mut self, attack: &Attack) {
        self.set_stat(&BaseStat::BrutaliteRetenue, 0);
        self.set_stat(&BaseStat::BrutaliteSevere, 0);
        let all_lines = attack.get_every_damage_lines();
        for (elem, damage, is_crit) in all_lines {
            let mut cur_brut = 0;
            cur_brut += self.get_stat(&elem.damage_based_on()) * 100;
            cur_brut += self.get_stat(&elem.stat_based_on()) * damage;
            cur_brut += self.get_stat(&BaseStat::DoMulti) * 100;
            cur_brut += self.get_stat(&BaseStat::Puissance) * damage;
            if attack.piege {
                cur_brut += self.get_stat(&BaseStat::DoPiege) * 100;
                cur_brut += self.get_stat(&BaseStat::PuissancePiege) * damage;
            }
            if is_crit {
                cur_brut += self.get_stat(&BaseStat::DoCri) * 100;
                self.set_stat(&BaseStat::BrutaliteSevere, self.get_stat(&BaseStat::BrutaliteSevere) + cur_brut / 100);
            } else {
                self.set_stat(&BaseStat::BrutaliteRetenue, self.get_stat(&BaseStat::BrutaliteRetenue) + cur_brut / 100);
            }
        }
        self.set_stat(&BaseStat::BrutaliteLocalisee, self.get_stat(
            if attack.damage_position == DamagePosition::Distance { &DoPerDist } else { &DoPerMelee }));
        self.set_stat(&BaseStat::BrutaliteMystique, self.get_stat(
            if attack.damage_source == DamageSource::Sort { &DoPerSo } else { &DoPerArme }));
    }

    pub fn base_stats(&self) -> [i64; 53] {
        self.base_stats
    }

    pub fn brutality_stats(&self) -> [i64; 6] {
        self.brutality_stats
    }
}