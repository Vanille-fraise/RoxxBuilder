use roxx_builder::builder::algorithm_mod::roxx_build_finder::RoxxBuildFinder;
use roxx_builder::builder::attack_mod::attack::Attack;
use roxx_builder::builder::attack_mod::damage_calculation::DamageCalculation::Max;
use roxx_builder::builder::attack_mod::damage_element::DamageElement::DamageAir;
use roxx_builder::builder::attack_mod::damage_line::DamageLine;
use roxx_builder::builder::attack_mod::damage_position::DamagePosition::Distance;
use roxx_builder::builder::attack_mod::damage_source::DamageSource::Sort;
use roxx_builder::builder::data_mod::data_loader::DataLoader;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_slot::ItemSlot::SlotCeinture;
use roxx_builder::builder::item_mod::item_type::ItemType;


static PRINT: bool = true;

#[test]
fn basic_build_test() { // todo: complete later
    let res_container = DataLoader::from_data_container_file("tests/test_files/containers/data_container_small_to_read".to_string());
    assert!(res_container.is_ok());
    let mut container = res_container.unwrap();
    let (mut best_item, bad_item1, bad_item2) = (Item::new_from_type(ItemType::Ceinture), Item::new_from_type(ItemType::Ceinture), Item::new_from_type(ItemType::Ceinture));
    let best_id = 123456;
    best_item.id = best_id;
    best_item.stats.insert(BaseStat::Agilite, 1000);
    container.items.push(best_item);
    container.items.push(bad_item1);
    container.items.push(bad_item2);
    container.clear_unknown_type();
    let roxx_builder = RoxxBuildFinder::new(&container);
    let spell = Attack::new(vec![DamageLine::new(DamageAir, 14, 24)], vec![DamageLine::new(DamageAir, 19, 29)], Sort, Distance, true, 20);
    let evaluation = roxx_builder.find_build(&spell, Max);
    assert!(evaluation.is_some());
    let ev = evaluation.unwrap();
    if PRINT { println!("Nb build tested: {} | Nb items in container: {} | Time: {}s", ev.build_evaluated, container.items.len(), ev.search_time.as_secs()); }
    assert_eq!(ev.build.items.get(&SlotCeinture).unwrap().id, best_id);
}