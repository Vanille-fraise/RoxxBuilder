use std::collections::HashSet;
use std::time::Instant;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::attack_mod::damage_calculation::DamageCalculation;
use crate::builder::build_mod::build::Build;
use crate::builder::build_mod::build_search_result::BuildSearchResult;
use crate::builder::data_mod::data_container::DataContainer;
use std::time::Duration;
use crate::builder::algorithm_mod::build_iter_factory::BuildIteratorFactory;

pub struct RoxxBuildFinder<'a> {
    data: DataContainer<'a>,
    build_iterator_factory: BuildIteratorFactory,
    pub time_limit: u128,
    calc_type: DamageCalculation,
    attack: Attack,
    pub track_data: bool,
}

impl<'a> RoxxBuildFinder<'a> {
    pub fn find_build(&self) -> BuildSearchResult {
        let now = Instant::now();
        let mut search_result = BuildSearchResult::new(i64::MIN, Build::new(), 0, Duration::new(0, 0), -1, None, -1);
        let mut best_build_id: [i64; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut seen: HashSet<String> = HashSet::new();
        let mut build_iterator = self.build_iterator_factory.create(&self.data, &self.attack);
        let mut opt_build = build_iterator.next_build();
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
                    search_result.spares += 1;
                }
            }
            search_result.build_evaluated += 1;
            if eval > search_result.eval {
                search_result.eval = eval;
                best_build_id = build.get_item_id();
                search_result.best_build_position = search_result.build_evaluated;
            }
            if self.time_limit > 0 && search_result.build_evaluated % 256 == 0
                && now.elapsed().as_nanos() > self.time_limit {
                break;
            }
            opt_build = build_iterator.next_build();
        }
        let mut final_build = Build::new();
        let fin_item_id = build_iterator.last_item_id();
        let mut fin_item = &self.data.items[0];
        for item in &self.data.items {
            for (cur_slot, cur_id) in best_build_id.iter().enumerate() {
                if *cur_id == item.id {
                    final_build.add_item(&item, num::FromPrimitive::from_usize(cur_slot).unwrap());
                    break;
                }
            }
            if fin_item_id.is_some() && item.id == fin_item_id.unwrap_or(-1) {
                fin_item = item;
            }
        }
        search_result.last_item_tested = Some(fin_item);
        search_result.build = final_build;
        search_result.search_time = now.elapsed();
        return search_result;
    }

    pub fn new(mut data: DataContainer<'a>, attack: Attack) -> Self {
        data.reset_brutality(&attack);
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