use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::builder::attack_mod::spell_variant::SpellVariant;
use crate::builder::{attack_mod::attack::Attack, player_mod::breed::Breed};
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::set::Set;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::builder::item_mod::item_type::ItemType;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataContainer{
    pub items: Vec<Item>,
    pub sets: Vec<Arc<Set>>,
    pub attacks: Vec<Arc<Attack>>,
    pub breeds: Vec<Breed>,
    pub spell_variants: Vec<SpellVariant>,
    pub version: String,
}

impl <'a> DataContainer{
    pub fn new() -> Self {
        DataContainer {
            items: vec![],
            sets: vec![],
            attacks: vec![],
            breeds: vec![],
            spell_variants: vec![],
            version: "0".to_string(),
        }
    }

    pub fn clear_unknown_type(&mut self) {
        let mut i = 0;
        while i < self.items.len() {
            if self.items[i].item_type == ItemType::Unknown {
                self.items.remove(i);
            } else { i += 1; }
        }
    }

    pub fn link_sub_data(&'a mut self) {
        let mut set_map: HashMap<i64, Arc<Set>> = HashMap::default();
        for set in self.sets.iter() {
            set_map.insert(set.id, set.clone());
        }
        for item in self.items.iter_mut() {
            if item.set_id > 0 && set_map.get(&item.set_id).is_some() {
                let cur_set = set_map.get(&item.set_id).unwrap();
                item.set = Some(cur_set.clone());
            }
        }
        let mut attack_map: HashMap<i64, Arc<Attack>> = HashMap::default();
        for attack in self.attacks.iter() {
            attack_map.insert(attack.id, attack.clone());
        }
        for spell_variant in self.spell_variants.iter_mut() {
            for spell_info in spell_variant.spells.iter_mut() {
                for attack_id in spell_info.spell_levels_id.clone() {
                    if attack_map.contains_key(&attack_id) {
                        spell_info.attacks.push(attack_map.get(&attack_id).unwrap().clone());
                    }
                }
            }
        }
    }
    
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn add_item_from_value(&mut self, value: Value) {
        let item = Item::from_serde_value(&value);
        self.items.push(item);
    }

    pub fn add_set(&mut self, set: Set) {
        self.sets.push(Arc::new(set));
    }

    pub fn reset_brutality(&'a mut self, attack: &Attack) {
        for item in self.items.iter_mut() {
            item.stats.reset_brutality(attack);
        }
        let mut new_sets: Vec<Arc<Set>> = vec![];
        for set in self.sets.iter() {
            let mut cur_set = set.deref().clone();
            for bonus in cur_set.bonus.iter_mut() {
                bonus.reset_brutality(attack);
            }
            new_sets.push(Arc::new(cur_set));
        }
        self.sets = new_sets;
        self.link_sub_data();
    }

    pub fn get_items_with_ids(&self, ids: &Vec<i64>) -> Vec<&Item> {
        let set_ids: Vec<i64> = self.sets.iter().filter(|s| ids.contains(&s.id)).map(|s| s.id).collect();
        self.items.iter().filter(|&i| set_ids.contains(&i.set.clone().map_or(0, |s| s.id)) || ids.contains(&i.id)).collect()
    }
    
    pub fn clear_not_breeds_attacks(&mut self) {
        let all_spell_levels_id: HashSet<&i64> = self.spell_variants.iter().flat_map(|variant| variant.spells.iter().flat_map(|spell_info| spell_info.spell_levels_id.iter())).collect();
        self.attacks.retain(|attack| all_spell_levels_id.contains(&attack.id));
    }
}