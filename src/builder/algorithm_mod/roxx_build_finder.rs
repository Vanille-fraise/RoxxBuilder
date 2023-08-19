use std::collections::HashSet;
use std::time::Instant;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::attack_mod::damage_calculation::DamageCalculation;
use crate::builder::build_mod::build::Build;
use crate::builder::build_mod::build_search_result::BuildSearchResult;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::algorithm_mod::build_iter_factory::BuildIteratorFactory;

pub struct RoxxBuildFinder {
    pub data: DataContainer,
    build_iterator_factory: BuildIteratorFactory,
    pub time_limit: u128,
    calc_type: DamageCalculation,
    attack: Attack,
    pub track_data: bool,
}

impl RoxxBuildFinder {
    pub fn find_build(&self) -> BuildSearchResult {
        let now = Instant::now();
        let mut search_result = BuildSearchResult::empty();
        let mut best_build_id: [i64; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut seen: HashSet<String> = HashSet::new();
        let mut build_iterator = self.build_iterator_factory.create(&self.data, &self.attack);
        let mut opt_build = build_iterator.next_build();
        let mut spares = 0;
        while opt_build.is_some() {
            let build = opt_build.unwrap();
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
            search_result.builds_evaluated += 1;
            if eval > search_result.eval {
                search_result.eval = eval;
                best_build_id = build.get_item_id();
                search_result.best_build_position = search_result.builds_evaluated;
            }
            if self.time_limit > 0 && search_result.builds_evaluated % 256 == 0
                && now.elapsed().as_nanos() > self.time_limit {
                break;
            }
            opt_build = build_iterator.next_build();
        }
        let mut final_build = Build::new();
        for item in &self.data.items {
            for (cur_slot, cur_id) in best_build_id.iter().enumerate() {
                if *cur_id == item.id {
                    final_build.add_item(&item, num::FromPrimitive::from_usize(cur_slot).unwrap());
                    break;
                }
            }
        }
        search_result.build = final_build;
        search_result.search_time = now.elapsed();
        search_result.additional_data.insert("spares".to_string(), spares.to_string());
        return search_result;
    }

    pub fn new(mut data: DataContainer, attack: Attack) -> Self {
        data.clear_unknown_type();
        data.reset_brutality(&attack);
        data.link_item_with_set();
        RoxxBuildFinder {
            data,
            build_iterator_factory: BuildIteratorFactory::new_iterative_factory(),
            time_limit: 30 * 1_000_000_000, // en_nano
            calc_type: DamageCalculation::Average,
            attack,
            track_data: true,
        }
    }

    pub fn set_attack(&mut self, attack: Attack) {
        self.attack = attack;
        self.set_brutality_estimation();
    }

    pub fn set_calc_type(&mut self, calc_type: DamageCalculation) {
        self.calc_type = calc_type;
        self.set_brutality_estimation();
    }

    fn set_brutality_estimation(&mut self) {
        self.data.reset_brutality(&self.attack);
    }

    pub fn get_data_container(&self) -> &DataContainer {
        &self.data
    }
}