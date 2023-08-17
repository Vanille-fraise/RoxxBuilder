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
        let mut build = Build::new_with_stats(Stats::from_map_stats(HashMap::from([(BaseStat::Puissance, 1400i64), (BaseStat::DoMulti, 160)]).iter()));
        let mut estimations = vec![];
        let used_slot = ItemSlot::SlotDofusPrysmaradite;
        for itm in &container.items {
            build.add_item(itm, used_slot.clone());
            estimations.push((build.evaluate_build_damage(attack), itm));
            build.remove_item(&used_slot);
        }
        estimations.sort_by(|e1, e2| e2.0.cmp(&e1.0));
        let mut by_cat: HashMap<ItemType, Vec<&Item>> = HashMap::new();
        ItemType::iter().for_each(|is| { by_cat.insert(is, vec![]); });
        estimations.iter().for_each(|x| { by_cat.get_mut(&x.1.item_type).unwrap().push(x.1); });
        while res.len() < container.items.len() {
            ItemType::iter().for_each(|i| {
                let v = by_cat.get_mut(&i).unwrap();
                (!v.is_empty()).then(|| res.push(v.remove(0)));
            });
        }
        return res;
    }
}