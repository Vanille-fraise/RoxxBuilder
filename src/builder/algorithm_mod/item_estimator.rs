use std::collections::HashMap;
use crate::builder::attack_mod::attack::Attack;
use crate::builder::build_mod::build::Build;
use crate::builder::data_mod::data_container::DataContainer;
use crate::builder::item_mod::base_stat_mod::base_stat::BaseStat;
use crate::builder::item_mod::item::Item;
use crate::builder::item_mod::item_slot::ItemSlot;


pub struct ItemEstimator;

impl ItemEstimator {
    pub fn roxx_based_estimation<'a>(container: &'a DataContainer, attack: &'a Attack) -> Vec<&'a Item<'a>> {
        let mut res = vec![];
        let mut build = Build::new_with(HashMap::from([(BaseStat::Puissance, 1400), (BaseStat::DoMulti, 160)]));
        let mut estimations = vec![];
        let used_slot = ItemSlot::SlotDofusPrysmaradite;
        for itm in &container.items {
            build.add_item(itm, used_slot.clone());
            estimations.push((build.evaluate_attack(attack).1, itm));
            build.remove_item(&used_slot);
        }
        estimations.sort_by(|e1, e2| e2.0.cmp(&e1.0));
        estimations.iter().for_each(|x| res.push(x.1));
        return res;
    }
}