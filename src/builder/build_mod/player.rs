use std::collections::HashMap;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SearchOptions {
    player_lvl: u64,
    player_class: String,
    player_stats: HashMap<BaseStat, i64>,
    enemy_stats: HashMap<BaseStat, i64>,
    conditions: HashMap<BaseStat, i64>,
    white_list: Vec<String>,
    black_list: Vec<String>,
}

impl SearchOptions {
    pub fn player_lvl(&self) -> u64 {
        self.player_lvl
    }
    pub fn player_class(&self) -> &str {
        &self.player_class
    }
    pub fn player_stats(&self) -> &HashMap<BaseStat, i64> {
        &self.player_stats
    }
    pub fn enemy_stats(&self) -> &HashMap<BaseStat, i64> {
        &self.enemy_stats
    }
    pub fn conditions(&self) -> &HashMap<BaseStat, i64> {
        &self.conditions
    }
    pub fn white_list(&self) -> &Vec<String> {
        &self.white_list
    }
    pub fn black_list(&self) -> &Vec<String> {
        &self.black_list
    }
}

