use std::collections::HashMap;
use strum::IntoEnumIterator;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::build_mod::build::Build;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::item_slot::ItemSlot;
use crate::builder::item_mod::item_type::ItemType;
use crate::builder::item_mod::stats::Stats;


pub struct ItemEstimator;

impl ItemEstimator {
    pub fn roxx_based_estimation<'a>(container: &'a DataContainer, attack: &Attack) -> Vec<&'a Item> {
        let mut res = vec![];
        let mut build = Build::new_with_stats(Stats::from_map_stats(HashMap::from([(BaseStat::Puissance, 1800i64), (BaseStat::DoMulti, 160)]).iter()));
        let mut estimations = vec![];
        let used_slot = ItemSlot::SlotDofusPrysmaradite;
        for itm in &container.items {
            build.add_item(itm, used_slot.clone());
            estimations.push((build.evaluate_build_damage(attack), itm));
            build.remove_item(&used_slot);
        }
        estimations.sort_by(|e1, e2| e2.0.cmp(&e1.0));
        let mut by_cat: HashMap<ItemSlot, Vec<&Item>> = HashMap::new();
        ItemSlot::iter().for_each(|is| { by_cat.insert(is, vec![]); });
        let mut by_cat_i: HashMap<ItemType, usize> = HashMap::new();
        ItemType::iter().for_each(|is| { by_cat_i.insert(is, 0); });
        for (_, itm) in estimations {
            let slots = ItemSlot::corresponding_to_item_type(&itm.item_type);
            by_cat.get_mut(&slots[by_cat_i.get(&itm.item_type).unwrap() % slots.len()]).unwrap().push(&itm);
            *by_cat_i.get_mut(&itm.item_type).unwrap() += 1;
        }
        while res.len() < container.items.len(){
            for (_, cur_items) in by_cat.iter_mut() {
                if !cur_items.is_empty(){
                    res.push(cur_items.remove(0));
                }
            }
        }
        return res;
    }
}