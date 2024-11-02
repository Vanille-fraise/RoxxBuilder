use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use serde::{Deserialize, Serialize};
use crate::builder::build_mod::build::Build;
use crate::builder::item_mod::item::Item;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum ItemCondition {
    MoreStatThan(BaseStat, i64),
    LessStatThan(BaseStat, i64),
    MoreAdditionalStatThan(BaseStat, i64),
    LessAdditionalStatThan(BaseStat, i64),
    StatEqualTo(BaseStat, i64),
    AdditionalStatEqualTo(BaseStat, i64),
    SetBonusLessThan(i64),
    And(Box<ItemCondition>, Box<ItemCondition>),
    Or(Box<ItemCondition>, Box<ItemCondition>),
    None,
}

impl ItemCondition {
    pub fn from_dofus_db_str(cond_str: &str) -> Self { // nasty function grrr, todo: add a line to check for not more than 2 bonus set
        if cond_str.contains('&') {
            let split: Vec<&str> = cond_str.splitn(2, '&').collect();
            return ItemCondition::And(Box::new(ItemCondition::from_dofus_db_str(split[0])), Box::new(ItemCondition::from_dofus_db_str(split[1])));
        } else if cond_str.contains('|') {
            let split: Vec<&str> = cond_str.splitn(2, '|').collect();
            return ItemCondition::Or(Box::new(ItemCondition::from_dofus_db_str(split[0])), Box::new(ItemCondition::from_dofus_db_str(split[1])));
        } else if cond_str.contains('>') {
            let split: Vec<&str> = cond_str.splitn(2, '>').collect();
            return if let (Some(elem), Ok(val)) = (BaseStat::from_str_repr(split[0]), split[1].parse::<i64>()) {
                if split[0].contains(char::is_lowercase) {
                    ItemCondition::MoreAdditionalStatThan(elem, val)
                } else { ItemCondition::MoreStatThan(elem, val) }
            } else { ItemCondition::None };
        } else if cond_str.contains('<') {
            let split: Vec<&str> = cond_str.splitn(2, '<').collect();
            return if let (Some(elem), Ok(val)) = (BaseStat::from_str_repr(split[0]), split[1].parse::<i64>()) {
                if split[0].contains(char::is_lowercase) {
                    ItemCondition::LessAdditionalStatThan(elem, val)
                } else { ItemCondition::LessStatThan(elem, val) }
            } else { ItemCondition::None };
        } else if cond_str.contains('=')/* || cond_str.contains('<') || cond_str.contains('=')*/ {
            let split: Vec<&str> = cond_str.splitn(2, '=').collect();
            return if let (Some(elem), Ok(val)) = (BaseStat::from_str_repr(split[0]), split[1].parse::<i64>()) {
                if split[0].contains(char::is_lowercase) {
                    ItemCondition::AdditionalStatEqualTo(elem, val)
                } else { ItemCondition::StatEqualTo(elem, val) }
            } else { ItemCondition::None };
        }
        ItemCondition::None
    }

    pub fn evaluate_soft_cond(&self, build: &Build, item: &Item, old_item: Option<&&Item>) -> bool {
        match self {
            // todo make a diff between additional and normal stats
            ItemCondition::And(c1, c2) => { c1.evaluate_soft_cond(build, item, old_item) && c2.evaluate_soft_cond(build, item, old_item) }
            ItemCondition::Or(c1, c2) => { c1.evaluate_soft_cond(build, item, old_item) || c2.evaluate_soft_cond(build, item, old_item) }
            ItemCondition::None => { true }
            ItemCondition::SetBonusLessThan(val) => { &(build.sets.iter().fold(0, |acc, (_, v)| -> usize { acc + v }) as i64) < val }
            ItemCondition::MoreStatThan(stat, val) => { &ItemCondition::eval(build, item, old_item, stat) > val }
            ItemCondition::LessStatThan(stat, val) => { &ItemCondition::eval(build, item, old_item, stat) < val }
            ItemCondition::MoreAdditionalStatThan(stat, val) => { &ItemCondition::eval(build, item, old_item, stat) > val }
            ItemCondition::LessAdditionalStatThan(stat, val) => { &ItemCondition::eval(build, item, old_item, stat) < val }
            ItemCondition::StatEqualTo(stat, val) => { &ItemCondition::eval(build, item, old_item, stat) == val }
            ItemCondition::AdditionalStatEqualTo(stat, val) => { &ItemCondition::eval(build, item, old_item, stat) == val }
        }
    }

    fn eval(build: &Build, cur_item: &Item, old_item: Option<&&Item>, stat: &BaseStat) -> i64 {
        let mut res = build.stats.get_stat(stat) + cur_item.stats.get_stat(stat);
        if let Some(itm) = old_item {
            res -= itm.stats.get_stat(stat);
        }
        res
    }
}
