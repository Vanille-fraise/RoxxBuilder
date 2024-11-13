use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    player_lvl: u64,
    player_class: String,
    player_stats: HashMap<BaseStat, i64>,
    enemy_stats: HashMap<BaseStat, i64>,
    conditions: HashMap<BaseStat, i64>,
    white_list: Vec<String>,
    black_list: Vec<String>,
    search_time_milli: u128,
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
    pub fn search_time_milli(&self) -> u128 {
        self.search_time_milli
    }
    pub fn empty() -> Self {
        SearchOptions {
            player_lvl: 200,
            player_class: "Eliotrope".to_string(),
            player_stats: Default::default(),
            enemy_stats: Default::default(),
            conditions: Default::default(),
            white_list: vec![],
            black_list: vec![],
            search_time_milli: 0,
        }
    }
    pub fn set_search_time_milli(&mut self, millisecond_time_limit: u128) {
        self.search_time_milli = millisecond_time_limit;
    }
    pub fn set_black_list(&mut self, black_list: Vec<String>) {
        self.black_list = black_list;
    }
}

