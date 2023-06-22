use std::cmp::max;
use serde::{Deserialize, Serialize};

extern crate serde;

use serde_big_array::BigArray;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
pub struct Stats {
    #[serde(with = "BigArray")]
    base_stats: [i64; 49],
    brutality_stats: [i64; 8],
}

impl Stats {
    pub const fn new_empty() -> Self {
        Stats {
            base_stats: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            brutality_stats: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
    pub fn from_effects_json_value(value: &serde_json::Value) -> Self {
        let mut stats = Stats::new_empty();
        value.as_array().unwrap_or(&mut vec![]).iter().filter(|v| { v["characteristic"].as_i64().unwrap_or(-1) > 0 }).for_each(|v| {
            let stat = BaseStat::from_dofus_db_val(v["characteristic"].as_i64().unwrap());
            let to = v["to"].as_i64().unwrap_or(0);
            let from = v["from"].as_i64().unwrap_or(0);
            stats.base_stats[stat as usize] = max(to, from);
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
}