use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashSet};
use std::collections::btree_map::BTreeMap;
use std::ops::{Not};
use std::sync::Arc;
use std::time;
use combinations::Combinations;
use itertools::Itertools;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::build_mod::build_search_result::BuildSearchResult;
use crate::builder::player_mod::player::SearchOptions;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::item_condition::ItemCondition;
use crate::builder::item_mod::item_slot::ItemSlot;
use crate::builder::item_mod::set::Set;
use crate::builder::item_mod::stats::Stats;

const EMPTY: u8 = 254;
const USED: u8 = 255;
const MIN_SEARCH_TIME: u128 = 1_000;

pub struct GraphLooker {
    slugs: [Vec<Slug>; 10],
    equipped_build: (Stats, [u8; 16]),
    visited: HashSet<[u8; 16]>,
    neighbors: BinaryHeap<(i64, [u8; 16])>,
    visits_per_slot: usize,
    computed_min_score: i64,
}

impl GraphLooker {
    pub fn search<'a>(container: &'a DataContainer, attack: &Attack, search_options: &SearchOptions) -> BuildSearchResult<'a> {
        let start = time::Instant::now();

        let search_time = max(MIN_SEARCH_TIME, search_options.search_time_milli());
        let mut res = BuildSearchResult::empty();
        let mut myself = Self::empty();
        myself.init(container, attack, search_options);
        myself.add_neighbors(attack, &mut res);
        let mut best_build: [u8; 16] = [EMPTY; 16];
        res.additional_data.insert("Preparation time".to_string(), format!("{:?}", start.elapsed()));
        while myself.equip_next_build() && start.elapsed().as_millis() < search_time {
            let equipped_damage = myself.equipped_build.0.evaluate_damage(attack);
            if equipped_damage > res.eval {
                res.eval = equipped_damage;
                res.best_build_position = res.builds_evaluated;
                best_build = myself.equipped_build.1.clone();
            }
            res.builds_evaluated += 1;
            myself.add_neighbors(attack, &mut res);
        }
        res.additional_data.insert("Final neighbors size".into(), myself.neighbors.len().to_string());
        let best_ids = best_build.iter().enumerate().map(|(item_type, item_pos)| (item_pos < &EMPTY).then(|| myself.slugs[Self::item_slot_to_slug_slot(item_type)][*item_pos as usize].item_ids.clone()).unwrap_or(vec![])).flatten().collect();
        container.get_items_with_ids(&best_ids).iter().enumerate().for_each(|(cur_slot, item)| {
            println!("{}", &item.name);
            res.build.add_item(&item, num::FromPrimitive::from_usize(cur_slot).unwrap());
        });
        res
    }

    fn item_slot_to_slug_slot(item_slot: usize) -> usize {
        if item_slot < 9 {
            item_slot
        } else if item_slot == 9 {
            8
        } else { 9 }
    }

    fn empty() -> Self {
        GraphLooker {
            slugs: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            equipped_build: (Stats::empty(), [EMPTY; 16]),
            visited: HashSet::with_capacity(2 ^ 20),
            neighbors: BinaryHeap::with_capacity(2 ^ 14),
            visits_per_slot: 1,
            computed_min_score: i64::MIN,
        }
    }

    fn equip_next_build(&mut self) -> bool {
        if self.neighbors.is_empty() { return false; }
        let next_build = self.neighbors.pop().unwrap();
        for i in 0..16 {
            if self.equipped_build.1[i] == next_build.1[i] { continue; }
            if self.equipped_build.1[i] < EMPTY {
                self.equipped_build.0.add_or_remove_brut_stats(&self.slugs[Self::item_slot_to_slug_slot(i)][self.equipped_build.1[i] as usize].stats, false);
            }
            if next_build.1[i] < EMPTY {
                self.equipped_build.0.add_or_remove_brut_stats(&self.slugs[Self::item_slot_to_slug_slot(i)][next_build.1[i] as usize].stats, true);
            }
        }
        self.equipped_build.1 = next_build.1.clone();
        true
    }

    fn add_neighbors(&mut self, attack: &Attack, build_search_result: &mut BuildSearchResult<'_>) {
        for (slot, &item_pos) in self.equipped_build.1.iter().enumerate() {
            if item_pos == USED { continue; }
            let mut cur_stat = self.equipped_build.0.clone();
            if item_pos < EMPTY {
                cur_stat.add_or_remove_brut_stats(&self.slugs[Self::item_slot_to_slug_slot(slot)][item_pos as usize].stats, false);
            }
            let eval = cur_stat.evaluate_damage(attack);
            let mut nb_visited = 0;
            let mut i = 0;
            while nb_visited < self.visits_per_slot && i < self.slugs[Self::item_slot_to_slug_slot(slot)].len() {
                // check if another slot is used
                if self.slugs[Self::item_slot_to_slug_slot(slot)][i].item_slots.iter().any(|&s| self.equipped_build.1[s] == USED) {
                    i += 1;
                    continue;
                }
                let mut cur_slots = self.equipped_build.1.clone();
                cur_slots[slot] = i as u8;
                // Check if the build has been visited
                // solution: do not add to neighbors
                // s2: remove the items/sets to equip it anyway
                if cur_slots[8] > cur_slots[9] {
                    (cur_slots[8], cur_slots[9]) = (cur_slots[9], cur_slots[8]);
                }
                cur_slots.split_at_mut(10).1.sort();
                self.slugs[Self::item_slot_to_slug_slot(slot)][i].item_slots.iter().for_each(|&s| cur_slots[s] = USED);
                if self.visited.insert(cur_slots.clone()) {
                    let to_add = (eval + self.slugs[Self::item_slot_to_slug_slot(slot)][i].score, cur_slots);
                    // todo only add neighbors if above a certain valid number ?
                    if self.check_condition(to_add, slot) {
                        self.neighbors.push(to_add);
                    }
                    nb_visited += 1;
                }
                build_search_result.builds_considered += 1;
                i += 1;
            }
        }
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
            for i in 2..slugs.len() {
                let combinations: Vec<Vec<usize>> = Combinations::new(slugs.clone(), i).collect();
                for cur_combinations in combinations {
                    all_slugs.push(Slug::from_set(cur_combinations.iter().map(|c| &all_slugs[*c]).collect(), set))
                }
            }
            all_slugs.push(Slug::from_set(slugs.iter().map(|c| &all_slugs[*c]).collect(), set))
        }
        self.attribute_slugs(all_slugs, attack);
    }

    fn attribute_slugs(&mut self, mut all_slugs: Vec<Slug>, attack: &Attack) {
        for slug in all_slugs.iter_mut() {
            slug.stats.reset_brutality(&attack);
            slug.score = slug.stats.evaluate_damage(attack) / max(slug.item_slots.len(), 1) as i64;
            // todo: maybe fix for dofus or ring, might be multiple spots
            // todo: score is added damage + base damage -> mess maybe with the eval
            // todo: some weapons are in multiple slots
        }
        all_slugs.sort_by(|e1, e2| e2.score.cmp(&e1.score));
        for slug in all_slugs.into_iter() {
            for (pos, &slot) in slug.item_slots.iter().enumerate() {
                if self.slugs[slot].len() < 254 {
                    let mut cur_slut = slug.clone();
                    cur_slut.item_slots.remove(pos);
                    self.slugs[slot].push(cur_slut);
                }
            }
        }
        // could improve with more precise score
        // todo: score that take condition into account
    }

    fn check_condition(&self, to_add: (i64, [u8; 16]), slot: usize) -> bool {
        if to_add.0 < self.computed_min_score { return false; }
        if slot >= 10 {
            // it is a trophee, dofus or prysma
            // *> todo to change, for testing purpose only
            for a in 10..16 {
                for b in 10..16 {
                    if a != b && to_add.1[a] < EMPTY && to_add.1[b] < EMPTY && to_add.1[a] == to_add.1[b] {
                        return false;
                    }
                }
            }
            let just_equipped = to_add.1[slot];
            for i in 10..16 {
                if
                i != slot && to_add.1[i] == just_equipped {
                    return false;
                }
            }
        }
        true
    }
}


#[derive(Eq, PartialEq, Clone)]
struct Slug {
    id: i64,
    item_slots: Vec<usize>,
    stats: Stats,
    condition: ItemCondition,
    score: i64,
    item_ids: Vec<i64>,
}

impl Slug {
    #[allow(dead_code)]
    fn empty() -> Self {
        Slug {
            id: 0,
            item_slots: vec![],
            stats: Stats::empty(),
            condition: ItemCondition::None,
            score: 0,
            item_ids: vec![],
        }
    }

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
            item_slots: ItemSlot::corresponding_to_item_type(&item.item_type).iter().map(Self::slot_to_usize).unique().collect(),
            stats: item.stats.clone(),
            condition: item.conditions.clone(),
            score: 0,
            item_ids: vec![item.id],
        }
    }

    fn from_set(slugs: Vec<&Slug>, set: &Set) -> Self {
        let mut stats = Stats::empty();
        slugs.iter().for_each(|s| stats.add_or_remove_stats(&s.stats, true));
        if slugs.len() > 0 && slugs.len() - 1 < set.bonus.len() {
            stats.add_or_remove_stats(&set.bonus[slugs.len() - 1], true);
        }
        let condition = slugs.iter().map(|slg| slg.condition.clone()).fold(ItemCondition::None, |cond, other| ItemCondition::And(cond.into(), other.clone().into()));
        Self {
            id: set.id,
            item_slots: slugs.iter().map(|s| s.item_slots.clone()).flatten().collect(),
            stats,
            condition,
            score: 0,
            item_ids: slugs.iter().map(|s| s.id).collect(),
        }
    }
}

fn is_compatible(item: &Item, search_options: &SearchOptions) -> bool {
    // todo handle conditions to not test items that needs incompatible conditions (ex. PA == 12 && PA <= 11)
    // handle white list
    search_options.player_lvl() >= item.lvl && search_options.black_list().iter().any(|s| &item.id.to_string() == s || &item.name == s).not()
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

