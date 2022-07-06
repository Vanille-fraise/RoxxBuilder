use crate::builder::item_mod::stat_mod::base_stat_mod::defensive_stat_mod;
use crate::builder::item_mod::stat_mod::base_stat_mod::offensive_stat_mod;

pub enum BaseStat {
    Defensive(offensive_stat_mod::offensive_stat::OffensiveStat),
    Offensive(defensive_stat_mod::defensive_stat::DefensiveStat)
}