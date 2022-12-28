use std::collections::HashMap;
use roxx_builder::builder::algorithm_mod::build_generator::BuildGenerator;
use roxx_builder::builder::item_mod::base_stat_mod::base_stat::BaseStat::Chance;
use roxx_builder::builder::item_mod::item::Item;
use roxx_builder::builder::item_mod::item_condition::ItemCondition;
use roxx_builder::builder::item_mod::item_type::ItemType::{Anneau, Cape, Ceinture, Dofus, Prysmaradite};
use roxx_builder::builder::item_mod::set::Set;

static NO_ITEM: Vec<&Item> = Vec::new();
static PRINT: bool = true;

fn test_nb_build(mut bg: BuildGenerator, expected_count: i32) {
    let mut count = 0;
    if PRINT {print!("\nBuild generated:\n");}
    while let Some(build) = bg.next_build() {
        count += 1;
        if PRINT {print!("{}\n", build.to_string());}
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
    test_nb_build(bg, 3);
}

#[test]
fn one_anneau_with_set_item_builds() {
    let mut loc_items = NO_ITEM.clone();
    let mut item = Item::new_from_type(Anneau);
    let set = Set::new(1, vec![HashMap::default(), HashMap::default()]);
    item.set_id = 1;
    item.set = Some(&set);
    loc_items.push(&item);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 2);
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
fn one_dofus_items_builds() {
    let mut loc_items = vec![];
    let item1 = Item::new_from_type(Dofus);
    loc_items.push(&item1);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 6);
}


#[test]
fn dofus_items_builds() {
    let mut loc_items = vec![];
    let item1 = Item::new_from_type(Dofus);
    let item2 = Item::new_from_type(Cape);
    loc_items.push(&item1);
    loc_items.push(&item2);
    let bg = BuildGenerator::new_with_items(loc_items);
    test_nb_build(bg, 13);
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
