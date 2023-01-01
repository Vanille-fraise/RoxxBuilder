use std::collections::HashSet;
use std::time::Instant;
use crate::builder::algorithm_mod::build_generator::BuildGenerator;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::attack_mod::damage_calculation::DamageCalculation;
use crate::builder::build_mod::build::Build;
use crate::builder::build_mod::damage_eval::DamageEval;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::set::Set;

#[allow(dead_code)]
pub struct RoxxBuildFinder<'a> {
    data: &'a DataContainer<'a>,
    pub estimator: Option<fn(container: &'a DataContainer, attack: &'a Attack) -> Vec<&'a Item<'a>>>,
    pub time_limit: u128,
    pub calc_type: DamageCalculation,
}

#[allow(dead_code)]
impl<'a> RoxxBuildFinder<'a> {
    pub fn find_build(&self, attack: &'a Attack) -> DamageEval { // well, could improve it
        let item_ref: Vec<&Item> = if let Some(f) = self.estimator
        { f(self.data, attack) } else { self.data.items.iter().collect() };
        let set_ref: Vec<&Set> = self.data.sets.iter().collect();
        let mut bg = BuildGenerator::new(item_ref, set_ref);
        let mut best_eval: (i64, i64, i64) = (i64::MIN, i64::MIN, i64::MIN);
        let mut best_build_id = vec![];
        let mut nb_evaluated_builds = 0;
        let mut spares = 0;
        let now = Instant::now();
        let mut seen: HashSet<i128> = HashSet::new();
        let mut damage_sum = 0i64;
        while let Some(build) = bg.next_build() {
            let eval = build.evaluate_attack(attack);
            nb_evaluated_builds += 1;
            damage_sum += eval.1;

            let ids_sum: i128 = build.items.iter().map(|i| i.1.id as i128).sum();
            if !seen.insert(ids_sum) { spares += 1; }
            if (self.calc_type == DamageCalculation::Min && eval.0 > best_eval.0)
                || (self.calc_type == DamageCalculation::Minimized && eval.0 > best_eval.0)
                || (self.calc_type == DamageCalculation::Average && eval.1 > best_eval.1)
                || (self.calc_type == DamageCalculation::Max && eval.2 > best_eval.2) {
                best_eval = eval;
                best_build_id = build.get_item_id();
            }
            if now.elapsed().as_nanos() > self.time_limit { break; }
        }
        let mut final_build = Build::new();
        for item in &self.data.items {
            for yay in &best_build_id {
                if yay.0 == item.id {
                    final_build.add_item(&item, yay.1);
                    break;
                }
            }
        }
        println!("Avg {}", damage_sum as f64 / nb_evaluated_builds as f64);
        DamageEval::new(best_eval, final_build, nb_evaluated_builds, now.elapsed(), spares)
    }

    pub fn new(data: &'a DataContainer<'a>) -> Self {
        RoxxBuildFinder {
            data,
            estimator: None,
            time_limit: 3 * 1_000_000_000, // en_nano
            calc_type: DamageCalculation::Average,
        }
    }
}