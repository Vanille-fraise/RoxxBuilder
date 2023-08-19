use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::ops::{Not};
use std::sync::Arc;
use combinations::Combinations;
use itertools::Itertools;
use sorted_vec::partial::SortedSet;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::build_mod::build_search_result::BuildSearchResult;
use crate::builder::build_mod::player::SearchOptions;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::item_condition::ItemCondition;
use crate::builder::item_mod::item_slot::ItemSlot;
use crate::builder::item_mod::set::Set;
use crate::builder::item_mod::stats::Stats;

pub struct GraphLooker {
    slugs: [Vec<Slug>; 10],
    equipped_build: (Stats, [u8; 16]),
    visited: HashSet<[u8; 16]>,
    neighbors: SortedSet<(i64, [u8; 16])>,
    visits_per_slot: usize,
}

impl GraphLooker {
    pub fn search<'a>(container: &'a DataContainer, attack: &Attack, search_options: &SearchOptions) -> BuildSearchResult<'a> {
        let mut res = BuildSearchResult::empty();
        let mut myself = Self::empty();
        myself.init(container, attack, search_options);
        myself.add_neighbors();
        let mut best_build: [u8; 16] = [255; 16];
        while myself.equip_next_build() {
            let equipped_damage = myself.equipped_build.0.evaluate_damage(attack);
            if equipped_damage > res.eval {
                res.eval = equipped_damage;
                res.best_build_position = res.builds_evaluated;
                best_build.copy_from_slice(&myself.equipped_build.1);
            }
            res.builds_evaluated += 1;
            myself.add_neighbors();
        }
        let best_ids: Vec<i64> = best_build.iter().enumerate()
            .map(|(slot, &pos)| myself.slugs[Self::item_slot_to_slug_slot(slot)][pos as usize].id).collect();
        for (cur_slot, item) in container.get_items_with_ids(&best_ids).iter().enumerate() {
            res.build.add_item(&item, num::FromPrimitive::from_usize(cur_slot).unwrap());
        }
        res
    }

    fn item_slot_to_slug_slot(item_slot: usize) -> usize {
        if item_slot < 9 {
            item_slot
        } else if item_slot == 9 {
            8
        } else { 10 }
    }

    fn empty() -> Self {
        GraphLooker {
            slugs: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            equipped_build: (Stats::new_empty(), [255; 16]),
            visited: HashSet::with_capacity(2 ^ 20),
            neighbors: SortedSet::with_capacity(2 ^ 14),
            visits_per_slot: 4,
        }
    }

    fn equip_next_build(&mut self) -> bool {
        if self.neighbors.is_empty() {
            return false;
        }
        let cur_build = self.neighbors.pop().unwrap();
        self.equipped_build.1.copy_from_slice(&cur_build.1);
        for i in 0..16 {
            if self.equipped_build.1[i] != 255 {
                self.equipped_build.0.add_or_remove_brut_stats(&self.slugs[Self::item_slot_to_slug_slot(i)][self.equipped_build.1[i] as usize].stats, false);
            }
            if cur_build.1[i] != 255 {
                self.equipped_build.0.add_or_remove_brut_stats(&self.slugs[i][cur_build.1[i] as usize].stats, true);
            }
        }
        true
    }

    fn add_neighbors(&mut self) {
        for (slot, &item_pos) in self.equipped_build.1.iter().enumerate() {
            let mut cur_stat = self.equipped_build.0.clone();
            if item_pos != 255 {
                cur_stat.add_or_remove_brut_stats(&self.slugs[Self::item_slot_to_slug_slot(slot)][item_pos as usize].stats, false);
            }
            let mut nb_visited = 0;
            let mut i = 0;
            while nb_visited < self.visits_per_slot && i < self.slugs[Self::item_slot_to_slug_slot(slot)].len() {
                // self.visited
                // todo: go trough the slugs, check if in visited. If not compute stats & add to neighbors
                i += 1;
            }
        }
        // all the fun is here :D
        // must:
        // - check condition
        // - prioritise when there is a condition
        // - limit the addition with limit size
        todo!()
    }

    fn init(&mut self, container: &DataContainer, attack: &Attack, search_options: &SearchOptions) {
        let mut all_slugs = vec![];
        let mut by_set: BTreeMap<Arc<Set>, Vec<usize>> = BTreeMap::new();
        for item in &container.items {
            if is_compatible(item, search_options) {
                let cur_slug = Slug::from_item(item);
                all_slugs.push(cur_slug);
                if let Some(set) = &item.set {
                    by_set.contains_key(set).not().then(|| by_set.insert(set.clone(), vec![]));
                    by_set.get_mut(set).unwrap().push(all_slugs.len() - 1);
                }
            }
        }
        for (set, slugs) in &by_set {
            for i in 0..slugs.len() {
                let combinations = Combinations::new(slugs.clone(), i);
                for cur_combinations in combinations {
                    all_slugs.push(Slug::from_set_slugs(cur_combinations.iter().map(|c| &all_slugs[*c]).collect(), set))
                }
            }
        }
        self.attribute_slugs(all_slugs, attack);
    }

    fn attribute_slugs(&mut self, mut all_slugs: Vec<Slug>, attack: &Attack) {
        for slug in all_slugs.iter_mut() {
            slug.score = slug.stats.evaluate_damage(attack) / slug.item_slots.len() as i64;
        }
        all_slugs.sort();
        for slug in all_slugs.into_iter() {
            for &slot in &slug.item_slots {
                if self.slugs[slot].len() < 255 {
                    self.slugs[slot].push(slug.clone());
                }
            }
        }
        // could improve with more precise score
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Slug {
    id: i64,
    item_slots: Vec<usize>,
    stats: Stats,
    condition: ItemCondition,
    score: i64,
}

impl Slug {
    /*
    fn empty() -> Self {
        Slug {
            id: 0,
            item_slots: vec![],
            stats: Stats::new_empty(),
            condition: ItemCondition::None,
            score: 0,
        }
    }*/

    fn slot_to_usize(slot: &ItemSlot) -> usize {
        match slot {
            ItemSlot::SlotAmulette => { 0 }
            ItemSlot::SlotBottes => { 1 }
            ItemSlot::SlotBouclier => { 2 }
            ItemSlot::SlotCape => { 3 }
            ItemSlot::SlotArme => { 4 }
            ItemSlot::SlotFamilierMonture => { 5 }
            ItemSlot::SlotCeinture => { 6 }
            ItemSlot::SlotChapeau => { 7 }
            ItemSlot::SlotAnneau1 | ItemSlot::SlotAnneau2 => { 8 }
            ItemSlot::SlotDofus2 | ItemSlot::SlotDofus3 | ItemSlot::SlotDofus4 |
            ItemSlot::SlotDofus5 | ItemSlot::SlotDofus6 | ItemSlot::SlotDofusPrysmaradite => { 9 }
        }
    }

    fn from_item(item: &Item) -> Self {
        Slug {
            id: item.id,
            item_slots: ItemSlot::corresponding_to_item_type(&item.item_type).iter().map(Self::slot_to_usize).collect(),
            stats: item.stats.clone(),
            condition: item.conditions.clone(),
            score: 0,
        }
    }

    fn from_set_slugs(slugs: Vec<&Slug>, set: &Set) -> Self {
        let mut stats = Stats::new_empty();
        slugs.iter().for_each(|s| stats.add_or_remove_stats(&s.stats, true));
        set.bonus.iter().for_each(|s| stats.add_or_remove_stats(&s, true));
        let condition = slugs.iter().map(|slg| slg.condition.clone()).fold(ItemCondition::None, |cond, other| ItemCondition::And(cond.into(), other.clone().into()));
        Self {
            id: set.id,
            item_slots: slugs.iter().map(|s| s.item_slots.clone()).flatten().unique().collect(),
            stats,
            condition,
            score: 0,
        }
    }
}

fn is_compatible(item: &Item, search_options: &SearchOptions) -> bool {
    // todo handle conditions to not test items that needs incompatible conditions (ex. PA == 12 && PA <= 11)
    search_options.player_lvl() <= item.lvl
}

impl PartialOrd<Self> for Slug {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Slug {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}
