use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum ItemCondition {
    MoreStatThan(BaseStat, i64),
    LessStatThan(BaseStat, i64),
}
