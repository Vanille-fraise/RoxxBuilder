use std::collections::HashMap;
use crate::builder::build_mod::build::Build;
use crate::builder::item_mod::item::Item;
use strum::IntoEnumIterator;
use crate::builder::item_mod::item_slot::ItemSlot;
use crate::builder::item_mod::set::Set;


pub struct BuildGenerator<'a> {
    items: Vec<&'a Item<'a>>,
    organized_items: HashMap<ItemSlot, Vec<&'a Item<'a>>>,
    sets: Vec<&'a Set>,
    organized_sets: HashMap<i64, &'a Set>,
    cur_build: Build<'a>,
    items_i: HashMap<ItemSlot, usize>,
    last_pushed_item: ItemSlot,
    cur_pushed_item_pos: usize,
}

#[allow(dead_code)]
impl<'a> BuildGenerator<'a> {
    pub fn new_with_items(items_to_build: Vec<&'a Item>) -> Self {
        let mut bg = BuildGenerator {
            items: items_to_build.clone(),
            organized_items: HashMap::new(),
            sets: vec![],
            organized_sets: Default::default(),
            cur_build: Build::new(),
            items_i: HashMap::new(),
            last_pushed_item: ItemSlot::SlotCape,
            cur_pushed_item_pos: 0,
        };
        bg.instantiate();
        bg
    }

    pub fn new(items_to_build: Vec<&'a Item>, sets: Vec<&'a Set>) -> Self {
        let mut bg = BuildGenerator {
            items: items_to_build.clone(),
            organized_items: HashMap::new(),
            sets,
            organized_sets: Default::default(),
            cur_build: Build::new(),
            items_i: HashMap::new(),
            last_pushed_item: ItemSlot::SlotCape,
            cur_pushed_item_pos: 0,
        };
        bg.instantiate();
        bg
    }


    fn instantiate(&mut self) {
        for set in &self.sets {
            self.organized_sets.insert(set.id, set);
        }
        for item_slot in ItemSlot::iter() {
            self.organized_items.insert(item_slot, vec![]);
            self.items_i.insert(item_slot, 0);
        }
    }

    fn push_item_to_organized(&mut self) -> bool {
        if self.items.is_empty() { return false; }
        let item = self.items.get(self.cur_pushed_item_pos).unwrap();
        for slot in ItemSlot::corresponding_to_item_type(&(*item).item_type) {
            if !self.organized_items.get_mut(&slot).unwrap().contains(item) {
                self.organized_items.get_mut(&slot).unwrap().push(item);
                self.last_pushed_item = slot.clone();
                return true;
            }
        }
        self.cur_pushed_item_pos += 1;
        if self.cur_pushed_item_pos == self.items.len() { return false; }

        let item = self.items.get(self.cur_pushed_item_pos).unwrap();
        let slot = ItemSlot::corresponding_to_item_type(&(*item).item_type).first().unwrap().clone();
        self.organized_items.get_mut(&slot).unwrap().push(item);
        self.last_pushed_item = slot.clone();
        true
    }

    pub fn next_build(&mut self) -> Option<&Build> {
        'main_loop: loop {
            for (t, i) in self.items_i.iter_mut() {
                if t == &self.last_pushed_item { continue; }
                if *i == self.organized_items.get(t).unwrap().len() {
                    *i = 0;
                    self.cur_build.remove_item(t);
                } else {
                    self.cur_build.add_item(self.organized_items.get(t).unwrap().get(*i).unwrap(), t.clone(), true);
                    *i += 1;
                    if self.cur_build.evaluate_build() {
                        return Some(&self.cur_build);
                    } else { continue 'main_loop; };
                }
            }
            if !self.push_item_to_organized() {
                return None;
            } else {
                ItemSlot::iter().for_each(|s| { self.cur_build.remove_item(&s); });
                self.cur_build.add_item(self.organized_items.get(&self.last_pushed_item).unwrap().last().unwrap(), self.last_pushed_item.clone(), true);
                self.items_i.iter_mut().for_each(|x| *x.1 = 0);
                if self.cur_build.evaluate_build() {
                    return Some(&self.cur_build);
                }
            }
        }
    }
}