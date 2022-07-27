use std::cmp::{max, min};
use std::collections::HashMap;
use string_builder::Builder;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::attack_mod::damage_calculation::DamageCalculation;
use crate::builder::attack_mod::damage_calculation::DamageCalculation::{Average, Max, Min, Minimized};
use crate::builder::attack_mod::damage_element::DamageElement;
use crate::builder::attack_mod::damage_position::DamagePosition;
use crate::builder::attack_mod::damage_source::DamageSource;
use crate::builder::build_mod::player::Player;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat::Critique;
use crate::builder::item_mod::item_slot::ItemSlot;
use crate::builder::item_mod::item_type::ItemType;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct Build<'a> {
    pub items: HashMap<ItemSlot, &'a Item<'a>>,
    pub stats: HashMap<BaseStat, i64>,
    player: Option<Player>,
    pub sets: HashMap<i64 /* id */, usize>,
}

#[allow(dead_code)]
impl<'a> Build<'a> {
    pub fn add_item(&mut self, item: &'a Item, item_slot: ItemSlot, force: bool) -> bool {
        if force || self.evaluate_item(item, &item_slot) {
            if self.items.get(&item_slot) != None {
                self.remove_item(&item_slot);
            }
            self.items.insert(item_slot, item);
            self.add_or_remove_stats(&item.stats, true);
            self.manage_set(item, true);
            true
        } else { false }
    }

    fn add_or_remove_stats(&mut self, stats: &HashMap<BaseStat, i64>, add: bool) {
        for stat in stats {
            if add {
                *self.stats.entry(*stat.0).or_insert(0) += stat.1;
            } else {
                *self.stats.entry(*stat.0).or_insert(0) -= stat.1;
            }
        }
    }

    fn evaluate_item(&self, item: &'a Item, slot: &ItemSlot) -> bool {
        if let Some(player) = self.player {
            if player.lvl < item.lvl { return false; }
        }
        if item.set_id > 0 || item.item_type != ItemType::Anneau {
            for it in &self.items {
                if it.0 != slot && it.1.id == item.id { return false; }
            }
        }
        item.conditions.evaluate(self, item, self.items.get(slot))
    }

    pub fn evaluate_build(&self) -> bool {
        for (slot, item) in &self.items {
            if !self.evaluate_item(item, slot) {
                return false;
            }
        }
        true
    }

    pub fn remove_item(&mut self, item_slot: &ItemSlot) -> bool {
        if !self.items.contains_key(&item_slot) { return false; }
        let item = self.items.remove(&item_slot).unwrap();
        self.manage_set(item, false);
        self.add_or_remove_stats(&item.stats, false);
        true
    }

    fn manage_set(&mut self, item: &Item, add: bool) {
        if item.set_id <= 0 { return; }
        if let Some(s) = item.set {
            let mut i = 0;
            if add {
                if self.sets.contains_key(&item.set_id) {
                    i = *self.sets.get(&item.set_id).unwrap();
                    self.add_or_remove_stats(&s.bonus[i], false); // remove the old bonus
                    i += 1;
                    *self.sets.entry(item.set_id).or_insert(0) += 1;
                } else {
                    self.sets.insert(item.set_id, 0);
                }
                self.add_or_remove_stats(&s.bonus[i], true); // add the new one
            } else if self.sets.contains_key(&item.set_id) {
                let mut key = *self.sets.get(&item.set_id).unwrap();
                if key > 0 {
                    self.add_or_remove_stats(&s.bonus[key], false);
                    key -= 1;
                    self.add_or_remove_stats(&s.bonus[key], true);
                    *self.sets.entry(item.set_id).or_insert(0) -= 1;
                } else {
                    self.add_or_remove_stats(&s.bonus[key], false);
                    self.sets.remove(&item.set_id);
                }
            }
        }
    }

    pub fn evaluate_attack(&self, attack: &Attack) -> (i64, i64, i64) { // todo: if the non crit damage could be higher than the crit damage
        let crit_chance = min(max(attack.base_crit + self.stats.get(&Critique).unwrap_or(&0), 0), 100);
        let min_eval = self.calculate_one_attack(attack, Min, attack.can_crit && crit_chance >= 100);
        let max_eval = self.calculate_one_attack(attack, Max, attack.can_crit && crit_chance > 0);
        let average_eval_low = self.calculate_one_attack(attack, Average, attack.can_crit && crit_chance >= 100);
        let average_eval_up = self.calculate_one_attack(attack, Average, attack.can_crit && crit_chance > 0);
        (min_eval, (average_eval_low * (100 - crit_chance) + average_eval_up * crit_chance) / 100, max_eval)
    }

    fn calculate_one_attack(&self, attack: &Attack, calc_type: DamageCalculation, make_crit: bool) -> i64 {
        let mut damage: i64 = 0;
        let damage_lines = if make_crit { &attack.crit_damages } else { &attack.damages };
        for damage_line in damage_lines {
            let value: i64 = match calc_type {
                Minimized => { damage_line.min_value }
                Min => { damage_line.min_value }
                Average => { (damage_line.min_value + damage_line.max_value) / 2 }
                Max => { damage_line.max_value }
            };
            match damage_line.damage_element {
                DamageElement::DamageAir => damage += self.one_value_damage(BaseStat::Agilite, BaseStat::DoAir, value, attack, make_crit),
                DamageElement::DamageTerre => damage += self.one_value_damage(BaseStat::Force, BaseStat::DoTerre, value, attack, make_crit),
                DamageElement::DamageEau => damage += self.one_value_damage(BaseStat::Chance, BaseStat::DoEau, value, attack, make_crit),
                DamageElement::DamageFeu => damage += self.one_value_damage(BaseStat::Intelligence, BaseStat::DoFeu, value, attack, make_crit),
                DamageElement::DamageNeutre => damage += self.one_value_damage(BaseStat::Force, BaseStat::DoNeutre, value, attack, make_crit),
            }
        };
        damage = damage * (100 + self.stats.get(&if attack.damage_source == DamageSource::Sort { BaseStat::DoPerSo } else { BaseStat::DoPerArme }).unwrap_or(&0)) / 100;
        damage = damage * (100 + self.stats.get(&if attack.damage_position == DamagePosition::Distance { BaseStat::DoPerDist } else { BaseStat::DoPerMelee }).unwrap_or(&0)) / 100;
        damage = damage * (100 + self.stats.get(&BaseStat::DoPerFinaux).unwrap_or(&0)) / 100;
        damage
    }

    fn one_value_damage(&self, stat: BaseStat, damage: BaseStat, damage_value: i64, attack: &Attack, make_crit: bool) -> i64 {
        let mut cur_damage: i64 = 0;
        cur_damage += self.stats.get(&damage).unwrap_or(&0) + self.stats.get(&BaseStat::DoMulti).unwrap_or(&0);
        cur_damage += ((self.stats.get(&stat).unwrap_or(&0) + self.stats.get(&BaseStat::Puissance).unwrap_or(&0)) / 100 + 1) * damage_value;
        if attack.piege {
            cur_damage += self.stats.get(&BaseStat::DoPiege).unwrap_or(&0);
            cur_damage += self.stats.get(&BaseStat::PuissancePiege).unwrap_or(&0) / 100 * damage_value;
        }

        if make_crit {
            cur_damage += self.stats.get(&BaseStat::DoCri).unwrap_or(&0);
        }

        cur_damage *= self.stats.get(&BaseStat::DoPerFinaux).unwrap_or(&0) / 100 + 1;
        cur_damage *= self.stats.get(if attack.damage_position == DamagePosition::Distance { &BaseStat::DoPerDist } else { &BaseStat::DoPerMelee }).unwrap_or(&0) / 100 + 1;
        cur_damage *= self.stats.get(if attack.damage_source == DamageSource::Sort { &BaseStat::DoPerSo } else { &BaseStat::DoPerArme }).unwrap_or(&0) / 100 + 1;
        cur_damage
    }

    pub fn new() -> Self {
        Build {
            items: HashMap::new(),
            stats: HashMap::new(),
            player: None,
            sets: Default::default(),
        }
    }

    pub fn new_with(stats: HashMap<BaseStat, i64>) -> Self {
        Build { items: HashMap::new(), stats, player: None, sets: Default::default() }
    }

    pub fn new_with_items(items: HashMap<ItemSlot, &'a Item>) -> Self {
        let mut build = Build { items: Default::default(), stats: Default::default(), player: None, sets: Default::default() };
        for data in items {
            build.add_item(data.1, data.0, false);
        }
        build
    }

    pub fn to_string(&self) -> String {
        let mut sb = Builder::new(255);
        sb.append("Build: [");
        let mut first = true;
        for item in &self.items {
            if !first { sb.append(", ") }
            first = false;
            sb.append(item.0.to_string());
            sb.append(':');
            sb.append(item.1.item_type.to_string())
        }
        sb.append("]");
        sb.string().unwrap_or("No item".to_string())
    }
    
    pub fn duplicate(&self) -> Build<'a> {
        Build{
            items: self.items.clone(),
            stats: self.stats.clone(),
            player: self.player.clone(),
            sets: self.sets.clone(),
        }
    }

    pub fn get_item_id(&self) -> Vec<(i64, ItemSlot)> {
        let mut res = vec![];
        self.items.iter().for_each(|(slt, itm)| {res.push((itm.id, slt.clone()))});
        res
    }
}

