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
}

#[allow(dead_code)]
impl<'a> RoxxBuildFinder<'a> {
    pub fn find_build(&self, attack: &Attack, calc_type: DamageCalculation) -> Option<DamageEval> { // well, could improve it
        let data_ref: Vec<&Item> = self.data.items.iter().collect();
        let set_ref: Vec<&Set> = self.data.sets.iter().collect();
        let mut bg = BuildGenerator::new(data_ref, set_ref);
        let mut best_eval: (i64, i64, i64) = (i64::MIN, i64::MIN, i64::MIN);
        let mut best_build_id = vec![];
        let mut nb_evaluated_builds = 0;
        let now = Instant::now();
        while let Some(build) = bg.next_build() {
            let eval = build.evaluate_attack(attack);
            nb_evaluated_builds += 1;
            if (calc_type == DamageCalculation::Min && eval.0 > best_eval.0)
                || (calc_type == DamageCalculation::Minimized && eval.0 > best_eval.0)
                || (calc_type == DamageCalculation::Average && eval.1 > best_eval.1)
                || (calc_type == DamageCalculation::Max && eval.2 > best_eval.2) {
                best_eval = eval;
                best_build_id = build.get_item_id();
            }
        }
        let mut final_build = Build::new();
        for item in &self.data.items {
            for yay in &best_build_id {
                if yay.0 == item.id {
                    final_build.add_item(&item, yay.1, true);
                    break;
                }
            }
        }
        Some(DamageEval::new(best_eval, final_build, nb_evaluated_builds, now.elapsed()))
    }

    pub fn new(data: &'a DataContainer<'a>) -> Self {
        RoxxBuildFinder {
            data
        }
    }
}