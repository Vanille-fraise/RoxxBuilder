use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter, Deserialize, Serialize, Display)]
pub enum ItemType {
    Amulette = 0,
    Anneau,
    Bottes,
    Bouclier,
    Cape,
    Ceinture,
    Chapeau,
    Dofus,
    Trophee,
    Prysmaradite,
    Arme,
}