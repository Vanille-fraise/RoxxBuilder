use crate::builder::item_mod::item_slot::ItemSlot::{SlotAmulette, SlotAnneau1, SlotAnneau2, SlotArme, SlotBottes, SlotBouclier, SlotCape, SlotCeinture, SlotChapeau, SlotDofus2, SlotDofus3, SlotDofus4, SlotDofus5, SlotDofus6, SlotDofusPrysmaradite, SlotFamilierMonture};
use crate::builder::item_mod::item_type::ItemType;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

extern crate num;

use num_derive::FromPrimitive;

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize, EnumIter, Display, PartialOrd, Ord, EnumCountMacro, FromPrimitive)]
pub enum ItemSlot {
    SlotAmulette = 0,
    SlotBottes,
    SlotBouclier,
    SlotCape,
    SlotArme,
    SlotFamilierMonture,
    SlotCeinture,
    SlotChapeau,
    SlotAnneau1,
    SlotAnneau2,
    SlotDofus2,
    SlotDofus3,
    SlotDofus4,
    SlotDofus5,
    SlotDofus6,
    SlotDofusPrysmaradite,
}

impl ItemSlot {
    pub fn corresponding_to_item_type(item_type: &ItemType) -> Vec<Self> {
        match item_type {
            ItemType::Amulette => { vec![SlotAmulette] }
            ItemType::Anneau => { vec![SlotAnneau1, SlotAnneau2] }
            ItemType::Bottes => { vec![SlotBottes] }
            ItemType::Bouclier => { vec![SlotBouclier] }
            ItemType::Cape => { vec![SlotCape] }
            ItemType::Ceinture => { vec![SlotCeinture] }
            ItemType::Chapeau => { vec![SlotChapeau] }
            ItemType::Dofus => { vec![SlotDofusPrysmaradite, SlotDofus2, SlotDofus3, SlotDofus4, SlotDofus5, SlotDofus6] }
            ItemType::Trophee => { vec![SlotDofusPrysmaradite, SlotDofus2, SlotDofus3, SlotDofus4, SlotDofus5, SlotDofus6] }
            ItemType::Prysmaradite => { vec![SlotDofusPrysmaradite] }
            ItemType::Familier => { vec![SlotFamilierMonture] }
            ItemType::Dragodinde => { vec![SlotFamilierMonture] }
            ItemType::Montilier => { vec![SlotFamilierMonture] }
            ItemType::Muldo => { vec![SlotFamilierMonture] }
            ItemType::Volkorne => { vec![SlotFamilierMonture] }
            ItemType::Arc => { vec![SlotArme] }
            ItemType::Baguette => { vec![SlotArme] }
            ItemType::Baton => { vec![SlotArme] }
            ItemType::Dague => { vec![SlotArme] }
            ItemType::Epee => { vec![SlotArme] }
            ItemType::Marteau => { vec![SlotArme] }
            ItemType::Pelle => { vec![SlotArme] }
            ItemType::Hache => { vec![SlotArme] }
            ItemType::Outil => { vec![SlotArme] }
            ItemType::Pioche => { vec![SlotArme] }
            ItemType::Faux => { vec![SlotArme] }
            ItemType::PierreDAme => { vec![SlotArme] }
            ItemType::FiletDeCapture => { vec![SlotArme] }
            ItemType::ArmeMagique => { vec![SlotArme] }
            ItemType::Unknown => { vec![] }
        }
    }
}