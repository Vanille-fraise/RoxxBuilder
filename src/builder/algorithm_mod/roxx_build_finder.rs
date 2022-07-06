use crate::builder::algorithm_mod::build_generator::BuildGenerator;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::attack_mod::damage_calculation::DamageCalculation;

#[allow(dead_code)]
pub struct RoxxBuildFinder<'a> {
    build_generator: BuildGenerator<'a>,
}

#[allow(dead_code)]
impl RoxxBuildFinder<'_> {
    pub fn find_build(&mut self, attack: &Attack, calc_type: DamageCalculation) -> i32 {
        // let mut best_build: Option<&Build> = None;
        let mut best_evaluation: i32 = -1;
        let mut cur_eval: i32;
        for build in self.build_generator.iter() {
            let att = build.evaluate_attack(attack);
            cur_eval = match calc_type {
                DamageCalculation::Minimized => { att.0 }
                DamageCalculation::Min => { att.0 }
                DamageCalculation::Average => { att.1 }
                DamageCalculation::Max => { att.2 }
            };
            if cur_eval > best_evaluation {
                best_evaluation = cur_eval;
            }
        }
        best_evaluation
    }
}