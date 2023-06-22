use roxx_builder::builder::algorithm_mod::build_generator::BuildGenerator;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::Chance;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_condition::ItemCondition;
use roxx_builder::builder::item_mod::item_type::ItemType::{Anneau, Arc, Cape, Ceinture, Dofus, Prysmaradite};
use roxx_builder::builder::item_mod::set::Set;
use roxx_builder::builder::item_mod::stats::Stats;

static NO_ITEM: Vec<&Item> = Vec::new();
static PRINT: bool = false;

fn test_nb_build(mut bg: BuildGenerator, expected_count: i32) {
    let mut count = 0;
    if PRINT { print!("\nBuild generated:\n"); }
    while let Some(build) = bg.next_build() {
        count += 1;
        if PRINT { print!("{}\n", build.to_string()); }
    }
    assert_eq!(expected_count, count);
}

#[test]
fn no_item_builds() {
    let bg = BuildGenerator::new_with_items(NO_ITEM.clone());
    test_nb_build(bg, 0);
}

#[test]
fn one_anneau_item_builds() {
    let mut loc_items = NO_ITEM.clone();
    let item = Item::new_from_type(Anneau);
    loc_items.push(&item);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 2);
}

#[test]
fn one_anneau_with_set_item_builds() {
    let mut loc_items = NO_ITEM.clone();
    let mut item = Item::new_from_type(Anneau);
    let set = Set::new(1, vec![Stats::new_empty(), Stats::new_empty()]);
    item.set_id = 1;
    item.set = Some(&set);
    loc_items.push(&item);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 1);
}

#[test]
fn multiple_anneau_with_set_item_builds() {
    let mut loc_items = NO_ITEM.clone();
    let mut item = Item::new_from_type(Anneau);
    let item_2 = Item::new_from_type(Anneau);
    let item_3 = Item::new_from_type(Arc);
    let set = Set::new(1, vec![Stats::new_empty(), Stats::new_empty()]);
    item.set_id = 1;
    item.name = "Set anneau".to_string();
    item.set = Some(&set);
    loc_items.push(&item_3);
    loc_items.push(&item_2);
    loc_items.push(&item);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 9);
}

#[test]
fn multiple_dofus_with_set_item_builds() {
    let mut loc_items = NO_ITEM.clone();
    let mut item = Item::new_from_type(Dofus);
    item.name = "1".to_string();
    let mut item_2 = Item::new_from_type(Dofus);
    item_2.name = "2".to_string();
    let item_3 = Item::new_from_type(Arc);
    loc_items.push(&item_3);
    loc_items.push(&item);
    loc_items.push(&item_2);
    let expected = 7;
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, expected);
    let bg = BuildGenerator::new_with_items(vec![&item, &item_3, &item_2]);
    test_nb_build(bg, expected);
}

#[test]
fn dofus_prisma_with_set_item_builds() {
    let mut loc_items = NO_ITEM.clone();
    let mut item = Item::new_from_type(Dofus);
    item.name = "1".to_string();
    let mut item_2 = Item::new_from_type(Prysmaradite);
    item_2.name = "2".to_string();
    let item_3 = Item::new_from_type(Arc);
    loc_items.push(&item_3);
    loc_items.push(&item_2);
    loc_items.push(&item);
    let expected = 7;
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, expected);
    let bg = BuildGenerator::new_with_items(vec![&item, &item_3, &item_2]);
    test_nb_build(bg, expected);
}

#[test]
fn multiple_anneau_with_dofus_set_item_builds() {
    let mut item_1 = Item::new_from_type(Anneau);
    let item_2 = Item::new_from_type(Anneau);
    let item_3 = Item::new_from_type(Arc);
    let mut item_4 = Item::new_from_type(Dofus);
    item_4.name = "4".to_string();
    let mut item_5 = Item::new_from_type(Prysmaradite);
    item_5.name = "5".to_string();
    let set = Set::new(1, vec![Stats::new_empty(), Stats::new_empty()]);
    item_1.set_id = 1;
    item_1.name = "1 & Set".to_string();
    item_1.set = Some(&set);
    let loc_items = vec![&item_1, &item_2, &item_3, &item_4, &item_5];
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 39);
    let loc_items = vec![&item_2, &item_5, &item_3, &item_4, &item_1];
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 39);
}

#[test]
fn one_ceinture_item_builds() {
    let mut loc_items = NO_ITEM.clone();
    let item = Item::new_from_type(Ceinture);
    loc_items.push(&item);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 1);
}

#[test]
fn two_items_builds() {
    let mut loc_items = NO_ITEM.clone();
    let item1 = Item::new_from_type(Ceinture);
    let item2 = Item::new_from_type(Cape);
    loc_items.push(&item1);
    loc_items.push(&item2);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 3);
}

#[test]
fn two_items_same_spot_builds() {
    let mut loc_items = NO_ITEM.clone();
    let item1 = Item::new_from_type(Ceinture);
    let item2 = Item::new_from_type(Ceinture);
    loc_items.push(&item1);
    loc_items.push(&item2);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 2);
}

#[test]
fn prisma_items_builds() {
    let mut loc_items = NO_ITEM.clone();
    let item1 = Item::new_from_type(Prysmaradite);
    let item2 = Item::new_from_type(Cape);
    loc_items.push(&item1);
    loc_items.push(&item2);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 3);
}

#[test]
fn
one_dofus_items_builds() {
    let mut loc_items = vec![];
    let item1 = Item::new_from_type(Dofus);
    loc_items.push(&item1);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 1);
}


#[test]
fn dofus_items_builds() {
    let mut loc_items = vec![];
    let item1 = Item::new_from_type(Dofus);
    let item2 = Item::new_from_type(Cape);
    loc_items.push(&item1);
    loc_items.push(&item2);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 3);
}

#[test]
fn build_with_impossible_item() {
    let mut loc_items = vec![];
    let mut item1 = Item::new_from_type(Dofus);
    item1.conditions = ItemCondition::AdditionalStatEqualTo(Chance, 500000);
    let item2 = Item::new_from_type(Cape);
    loc_items.push(&item1);
    loc_items.push(&item2);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 1);
}
