use std::collections::HashMap;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_element::DamageElement::{DamageAir, DamageEau, DamageFeu};
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::build_mod::build::Build;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::{Agilite, Chance, DoAir, DoCri, Intelligence};
use roxx_builder::builder::item_mod::stats::Stats;

fn test_crit(build_stats: HashMap<BaseStat, i64>, attack_lines: Vec<DamageLine>, crit_attack_lines: Vec<DamageLine>, crit_chance: i64, expected_damage: (i64, i64, i64)) {
    let build = Build::new_with_stats(Stats::from_map_stats(build_stats.iter()));
    let mut attack = Attack::default();
    attack.can_crit = true;
    attack.damages = attack_lines;
    attack.crit_damages = crit_attack_lines;
    attack.base_crit = crit_chance;

    assert_eq!(build.evaluate_attack(&attack), expected_damage)
}

#[test]
fn no_crit_damage_test() {
    test_crit(HashMap::from([(Chance, 100), (DoCri, 100)]), vec![DamageLine::new_fix(DamageEau, 20)], vec![DamageLine::new_fix(DamageEau, 30)], 0, (40, 40, 40));
}

#[test]
fn negative_crit_chance_test(){
    test_crit(HashMap::from([(Chance, 100), (DoCri, 100)]), vec![DamageLine::new_fix(DamageEau, 20)], vec![DamageLine::new_fix(DamageEau, 30)], -50, (40, 40, 40));

}

#[test]
fn half_crit_chance_damage_test() {
    test_crit(HashMap::from([(Chance, 100), (DoCri, 100)]), vec![DamageLine::new_fix(DamageEau, 20)], vec![DamageLine::new_fix(DamageEau, 30)], 50, (40, 100, 160));
}

#[test]
fn hundred_percent_crit_test() {
    test_crit(HashMap::from([(Agilite, 300)]), vec![DamageLine::new_fix(DamageAir, 10)], vec![DamageLine::new_fix(DamageAir, 50)], 150, (200, 200, 200));
}

#[test]
fn ten_percent_crit_test() {
    test_crit(HashMap::from([(Agilite, 300), (DoAir, 10)]), vec![DamageLine::new_fix(DamageAir, 20)], vec![DamageLine::new_fix(DamageAir, 50)], 10, (90, 102, 210));
}

#[test]
fn eighty_percent_crit_test() {
    test_crit(HashMap::from([(Agilite, 300), (DoAir, 10)]), vec![DamageLine::new_fix(DamageAir, 20)], vec![DamageLine::new_fix(DamageAir, 50)], 80, (90, 186, 210));
}

#[test]
fn different_min_max_test(){
    test_crit(HashMap::from([(Agilite, 900), (DoCri, 100)]), vec![DamageLine::new(DamageAir, 20, 30)], vec![DamageLine::new(DamageAir, 40, 50)], 80, (200, 490, 600));

}

#[test]
fn completely_different_crit_line_test(){
    test_crit(HashMap::from([(Agilite, 300), (DoAir, 10), (Intelligence, 100)]), vec![DamageLine::new_fix(DamageAir, 20)], vec![DamageLine::new_fix(DamageFeu, 50),DamageLine::new_fix(DamageFeu, 100)], 40, (90, 174, 300));
}

