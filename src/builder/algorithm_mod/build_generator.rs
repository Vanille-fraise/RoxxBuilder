use std::collections::HashMap;
use crate::builder::build_mod::build::Build;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::item_slot::ItemSlot;
use crate::builder::item_mod::set::Set;
use strum::{EnumCount, IntoEnumIterator};


pub struct BuildGenerator<'a> {
    items: Vec<&'a Item<'a>>,
    organized_items: Vec<Vec<&'a Item<'a>>>,
    sets: Vec<&'a Set>,
    organized_sets: HashMap<i64, &'a Set>,
    cur_build: Build<'a>,
    items_i: Vec<usize>,
    last_pushed_item: ItemSlot,
    cur_pushed_item_pos: usize,

}

#[allow(dead_code)]
impl<'a> BuildGenerator<'a> {
    pub fn new_with_items(items_to_build: Vec<&'a Item>) -> Self {
        let mut bg = BuildGenerator {
            items: items_to_build.clone(),
            organized_items: vec![],
            sets: vec![],
            organized_sets: Default::default(),
            cur_build: Build::new(),
            items_i: vec![],
            last_pushed_item: ItemSlot::SlotAmulette,
            cur_pushed_item_pos: 0,
        };
        bg.instantiate();
        bg
    }

    pub fn new(items_to_build: Vec<&'a Item>, sets: Vec<&'a Set>) -> Self {
        let mut bg = BuildGenerator {
            items: items_to_build.clone(),
            organized_items: vec![],
            sets,
            organized_sets: Default::default(),
            cur_build: Build::new(),
            items_i: vec![],
            last_pushed_item: ItemSlot::SlotAmulette,
            cur_pushed_item_pos: 0,
        };
        bg.instantiate();
        bg
    }


    fn instantiate(&mut self) {
        for set in &self.sets {
            self.organized_sets.insert(set.id, set);
        }
        let nb_slot: usize = ItemSlot::COUNT;
        for _i in 0..nb_slot {
            self.organized_items.push(vec![]);
            self.items_i.push(0);
        }
    }

    fn push_item_to_organized(&mut self) -> bool {
        loop {
            if self.cur_pushed_item_pos == self.items.len() { return false; }
            let item = self.items.get(self.cur_pushed_item_pos).unwrap();
            for slot in ItemSlot::corresponding_to_item_type(&(*item).item_type) {
                if !self.organized_items.get_mut(slot as usize).unwrap().contains(item) {
                    self.organized_items.get_mut(slot as usize).unwrap().push(item);
                    self.last_pushed_item = slot.clone();
                    return true;
                }
            }
            self.cur_pushed_item_pos += 1;
        }
    }

    pub fn next_build(&mut self) -> Option<&Build> {
        'main_loop: loop {
            for type_i in 0..self.items_i.len() {
                let i = self.items_i.get_mut(type_i).unwrap();
                let itm_slt: ItemSlot = num::FromPrimitive::from_usize(type_i).unwrap();
                if itm_slt == self.last_pushed_item { continue; }
                while *i < self.organized_items.get(type_i).unwrap().len() {
                    if !self.cur_build.add_item(self.organized_items.get(type_i).unwrap().get(*i).unwrap(), itm_slt.clone()) {
                        *i += 1;
                        continue;
                    }
                    *i += 1;
                    if self.cur_build.evaluate_soft_cond_build() {
                        return Some(&self.cur_build);
                    } else { continue 'main_loop; };
                }
                *i = 0;
                self.cur_build.remove_item(&itm_slt);
            }
            if !self.push_item_to_organized() {
                return None;
            } else {
                ItemSlot::iter().for_each(|s| { self.cur_build.remove_item(&s); });
                self.cur_build.add_item(self.organized_items.get(self.last_pushed_item as usize).unwrap().last().unwrap(), self.last_pushed_item.clone());
                self.items_i.iter_mut().for_each(|x| *x = 0);
                if self.cur_build.evaluate_soft_cond_build() {
                    return Some(&self.cur_build);
                }
            }
        }
    }
}