use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
extern crate num;
use num_derive::FromPrimitive;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter, Deserialize, Serialize, Display, FromPrimitive)]
pub enum ItemType {
    Unknown=0,
    Amulette = 1,
    Arc = 2,
    Baguette = 3,
    Baton = 4,
    Dague = 5,
    Epee = 6,
    Marteau = 7,
    Pelle = 8,
    Anneau = 9,
    Ceinture = 10,
    Bottes = 11,
    Chapeau = 16,
    Cape = 17,
    Familier = 18,
    Hache = 19,
    Outil = 20,
    Pioche = 21,
    Faux = 22,
    Dofus = 23,
    Bouclier = 82,
    PierreDAme = 83,
    Dragodinde = 97,
    FiletDeCapture = 99,
    // ??? 102 ??? supposément dans le slot d'arme: suggestion arbalète
    ArmeMagique = 114,
    Montilier = 121,
    Trophee = 151,
    Muldo = 196,
    Volkorne = 207,
    Prysmaradite = 217,
}