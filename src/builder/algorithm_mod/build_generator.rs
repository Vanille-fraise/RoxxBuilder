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
        for item in &self.items {
            for slot in ItemSlot::corresponding_to_item_type(&(*item).item_type) {
                self.organized_items.get_mut(&slot).unwrap().push(item);
            }
        }
    }

    pub fn next_build(&mut self) -> Option<&Build> { // todo: change it to not skip unequipped item
        'main_loop: for (t, i) in self.items_i.iter_mut() {
            'cannot_equip_loop: loop {
                if *i >= self.organized_items.get(t).unwrap().len() {
                    *i = 0;
                    self.cur_build.remove_item(t);
                    continue 'main_loop;
                } else {
                    let could_equip = self.cur_build.add_item(self.organized_items.get(t).unwrap().get(*i).unwrap(), t.clone(), false);
                    *i += 1;
                    if !could_equip { continue 'cannot_equip_loop; } else { return Some(&self.cur_build); }
                }
            }
        }
        return None;
    }
}