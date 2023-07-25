use std::collections::HashSet;
use std::time::Instant;
use crate::builder::algorithm_mod::build_generator::BuildGenerator;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::attack_mod::damage_calculation::DamageCalculation;
use crate::builder::build_mod::build::Build;
use crate::builder::build_mod::build_search_result::BuildSearchResult;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::set::Set;


#[allow(dead_code)]
pub struct RoxxBuildFinder<'a> {
    data: DataContainer<'a>,
    pub estimator: Option<fn(container: &'a DataContainer, attack: &'a Attack) -> Vec<&'a Item<'a>>>,
    pub time_limit: u128,
    calc_type: DamageCalculation,
    attack: &'a Attack,
    pub track_data: bool,
}

#[allow(dead_code)]
impl<'a> RoxxBuildFinder<'a> {
    pub fn find_build(&'a self) -> BuildSearchResult {
        let item_ref: Vec<&Item> = if let Some(f) = self.estimator
        { f(&self.data, &self.attack) } else { self.data.items.iter().collect() };
        let set_ref: Vec<&Set> = self.data.sets.iter().collect();
        let mut bg = BuildGenerator::new(item_ref, set_ref);
        let mut best_eval = i64::MIN;
        let mut best_build_id: [i64; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut nb_evaluated_builds = 0;
        let mut spares = 0;
        let mut seen: HashSet<String> = HashSet::new();
        let now = Instant::now();
        while let Some(build) = bg.next_build() {
            let eval = build.evaluate_build_damage(&self.attack);
            if self.track_data {
                let mut pos = 0;
                let ids_concat = build.items.iter().map(|item| { item.id.to_string() }).fold("".to_string(), |mut s1, s2| {
                    s1.push_str(&pos.to_string());
                    pos += 1;
                    s1.push_str(&s2);
                    s1
                });
                if !seen.insert(ids_concat.clone()) {
                    spares += 1;
                }
            }
            nb_evaluated_builds += 1;
            if eval > best_eval {
                best_eval = eval;
                best_build_id = build.get_item_id();
            }
            if self.time_limit > 0 && nb_evaluated_builds % 256 == 0
                && now.elapsed().as_nanos() > self.time_limit {
                break;
            }
        }
        let mut final_build = Build::new();
        let fin_item_id = bg.get_last_item_id();
        let mut fin_item = &self.data.items[0];
        for item in &self.data.items {
            for (cur_slot, cur_id) in best_build_id.iter().enumerate() {
                if *cur_id == item.id {
                    final_build.add_item(&item, num::FromPrimitive::from_usize(cur_slot).unwrap());
                    break;
                }
            }
            if item.id == fin_item_id {
                fin_item = item;
            }
        }
        return BuildSearchResult::new(best_eval, final_build, nb_evaluated_builds, now.elapsed(), spares, fin_item);
    }

    pub fn new(mut data: DataContainer<'a>, attack: &'a Attack) -> Self {
        data.reset_brutality(attack);
        RoxxBuildFinder {
            data,
            estimator: None,
            time_limit: 180 * 1_000_000_000, // en_nano
            calc_type: DamageCalculation::Average,
            attack,
            track_data: true,
        }
    }

    pub fn set_attack(&mut self, attack: &'a Attack) {
        self.set_brutality_estimation(attack, self.calc_type);
    }

    pub fn set_calc_type(&mut self, calc_type: DamageCalculation) {
        self.set_brutality_estimation(self.attack, calc_type);
    }

    pub fn set_brutality_estimation(&mut self, attack: &'a Attack, calc_type: DamageCalculation) {
        self.attack = attack;
        self.calc_type = calc_type;
        self.data.reset_brutality(attack);
    }

    pub fn get_data_container(&self) -> &DataContainer {
        &self.data
    }
}