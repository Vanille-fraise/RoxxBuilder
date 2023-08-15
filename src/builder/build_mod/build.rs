use std::cmp::{max, min};
use std::collections::HashMap;
use num_traits::FromPrimitive;
use string_builder::Builder;
use strum::IntoEnumIterator;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::attack_mod::damage_calculation::DamageCalculation::{Average, Max, Min, Minimized};
use crate::builder::build_mod::player::Player;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat::{BrutaliteRetenue, BrutaliteSevere, Critique};
use crate::builder::item_mod::item;
use crate::builder::item_mod::item_slot::ItemSlot;
use crate::builder::item_mod::item_type::ItemType;
use crate::builder::item_mod::stats::Stats;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub struct Build<'a> {
    pub items: [&'a Item; 16],
    pub stats: Stats,
    player: Option<Player>,
    pub sets: HashMap<i64 /* id */, usize>,
}

#[allow(dead_code)]
impl<'a> Build<'a> {
    pub fn add_item(&mut self, item: &'a Item, item_slot: ItemSlot) -> bool {
        if self.evaluate_hard_cond_and_compatibility_item(item, &item_slot) {
            self.remove_item(&item_slot);
            self.items[item_slot as usize] = item;
            self.stats.add_or_remove_stats(&item.stats, true);
            self.manage_set(item, true);
            return true;
        }
        return false;
    }

    fn evaluate_hard_cond_and_compatibility_item(&self, item: &'a Item, slot: &ItemSlot) -> bool {
        if let Some(player) = self.player {
            if player.lvl < item.lvl { return false; }
        }
        if item.set_id > 0 || item.item_type != ItemType::Anneau {
            for (n, it) in self.items.iter().enumerate() {
                if n != *slot as usize && it.id == item.id { return false; }
            }
        }
        return true;
    }

    pub fn evaluate_soft_cond_build(&self) -> bool {
        /* for (slot, item) in &self.items {
            if !self.evaluate_hard_cond_and_compatibility_item(item, slot) {
                return false;
            }
        } */
        for item in &self.items {
            if !item.conditions.evaluate_soft_cond(self, item, None) { return false; }
        }
        true
    }

    pub fn remove_item(&mut self, item_slot: &ItemSlot) -> bool {
        if self.items[*item_slot as usize].id <= 16 { return false; }
        self.stats.add_or_remove_stats(&self.items[*item_slot as usize].stats, false);
        self.manage_set(&self.items[*item_slot as usize], false);
        self.items[*item_slot as usize] = &item::EMPTY_ITEMS[0];
        true
    }

    fn manage_set(&mut self, item: &Item, add: bool) {
        if item.set_id <= 0 { return; }
        if let Some(s) = &item.set {
            let mut i = 0;
            if add {
                if self.sets.contains_key(&item.set_id) {
                    i = *self.sets.get(&item.set_id).unwrap();
                    self.stats.add_or_remove_stats(&s.bonus[i], false); // remove the old bonus
                    i += 1;
                    *self.sets.entry(item.set_id).or_insert(0) += 1;
                } else {
                    self.sets.insert(item.set_id, 0);
                }
                self.stats.add_or_remove_stats(&s.bonus[i], true); // add the new one
            } else if self.sets.contains_key(&item.set_id) {
                let mut key = *self.sets.get(&item.set_id).unwrap();
                if key > 0 {
                    self.stats.add_or_remove_stats(&s.bonus[key], false);
                    key -= 1;
                    self.stats.add_or_remove_stats(&s.bonus[key], true);
                    *self.sets.entry(item.set_id).or_insert(0) -= 1;
                } else {
                    self.stats.add_or_remove_stats(&s.bonus[key], false);
                    self.sets.remove(&item.set_id);
                }
            }
        }
    }

    pub fn evaluate_build_damage(&self, attack: &Attack) -> i64 {
        let crit = match (attack.damage_calculation(), attack.can_crit) {
            (_, false) | (Minimized, true) => 0,
            (Min, true) => if self.stats.get_stat(&Critique) + attack.base_crit < 100 { 0 } else { 100 },
            (Average, true) => max(min(self.stats.get_stat(&Critique) + attack.base_crit, 100), 0),
            (Max, true) => if self.stats.get_stat(&Critique) + attack.base_crit > 0 { 100 } else { 0 },
        };
        let mut damage: i64 = match crit {
            0 => self.stats.get_stat(&BrutaliteRetenue) + attack.brutality_damage(),
            100 => self.stats.get_stat(&BrutaliteSevere) + attack.brutality_crit_damage(),
            _ => ((self.stats.get_stat(&BrutaliteSevere) + attack.brutality_crit_damage()) * crit + (self.stats.get_stat(&BrutaliteRetenue) + attack.brutality_damage()) * (100 - crit)) / 100,
        };
        damage *= (100 + self.stats.get_stat(&BaseStat::BrutaliteLocalisee)) * (100 + self.stats.get_stat(&BaseStat::DoPerFinaux)) * (100 + self.stats.get_stat(&BaseStat::BrutaliteMystique));
        damage /= 100 * 100 * 100;
        damage
    }
    pub fn new() -> Self {
        Build {
            items: Item::ref_empty_items(),
            stats: Stats::new_empty(),
            player: None,
            sets: Default::default(),
        }
    }

    pub fn new_with_stats(stats: Stats) -> Self {
        Build { items: Item::ref_empty_items(), stats, player: None, sets: Default::default() }
    }

    pub fn new_with_items(items: [&'a Item; 16]) -> Self {
        let mut build = Build::new();
        for (i, item) in items.iter().enumerate() {
            if item.id > 15 {
                build.add_item(item, ItemSlot::from_usize(i).unwrap());
            }
        }
        build
    }

    pub fn new_with_item_map(items_map: &'a HashMap<ItemSlot, &Item>) -> Self {
        let mut clone_of_base_item : [&Item; 16]= Item::ref_empty_items();
        for (slot, item) in items_map {
            clone_of_base_item[*slot as usize] = item;
        }
        Self::new_with_items(clone_of_base_item)
    }

    pub fn to_string(&self) -> String {
        let mut sb = Builder::new(255);
        sb.append("Build: [");
        let mut first = true;
        for item in &self.items {
            if !first { sb.append(", ") }
            first = false;
            sb.append(item.item_type.to_string());
            if !item.name.is_empty() && item.name != "No name" {
                sb.append("-");
                sb.append(item.name.clone())
            };
        }
        sb.append("]");
        sb.string().unwrap_or("No item".to_string())
    }

    pub fn duplicate(&self) -> Build<'a> {
        Build {
            items: self.items.clone(),
            stats: self.stats.clone(),
            player: self.player.clone(),
            sets: self.sets.clone(),
        }
    }

    pub fn get_item_id(&self) -> [i64; 16] {
        self.items.map(|itm| { itm.id })
    }

    /// function to use ONLY for test purposes, never use it in real situation
    pub fn recompute_all_stats(&mut self) {
        BaseStat::iter().for_each(|bs| self.compute_one_stat(&bs));
    }
    /// function to use ONLY for test purposes, never use it in real situation
    fn compute_one_stat(&mut self, stat: &BaseStat) {
        self.stats.set_stat(stat, self.items.map(|item| item.stats.get_stat(stat)).iter().sum());
    }
}
