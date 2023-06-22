use serde::{Deserialize, Serialize};
use num_derive::FromPrimitive;
use strum_macros::EnumIter;

extern crate num;

#[repr(usize)]
#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize, Debug, FromPrimitive, EnumIter, PartialOrd, Ord)]
pub enum BaseStat {
    // my_item[effects][i][characteristic]
    PA = 0,
    PM,
    Po,
    Invo,

    Force,
    Chance,
    Agilite,
    Intelligence,
    Puissance,
    PuissancePiege,
    Sagesse,
    Vitalite,

    DoMulti,
    DoPiege,
    DoEau,
    DoAir,
    DoFeu,
    DoTerre,
    DoNeutre,
    DoCri,
    DoPou,

    Tacle,
    Fuite,
    EsqPA,
    EsqPM,
    RetPA,
    RetPM,

    RePerEau,
    RePerNeutre,
    RePerFeu,
    RePerAir,
    RePerTerre,

    ReFixEau,
    ReFixNeutre,
    ReFixFeu,
    ReFixAir,
    ReFixTerre,
    ReCri,
    RePou,

    RePerDist,
    RePerMelee,
    RePerSo,
    RePerArme,

    Soin,
    Initiative,
    Prospection,
    RenvoiDo,

    Carac29,
    // idk what that is but some item has it
    Unknown,

    //created Roxx values

    // brutality for non-critical hit
    BrutaliteRetenue,
    // brutality for critical hit
    BrutaliteSevere,
    Critique,
    DoPerMelee,
    DoPerDist,
    DoPerArme,
    DoPerSo,
    DoPerFinaux, // seems like its not a stat
}

impl BaseStat {
    pub fn from_str_repr(cond_str: &str) -> Option<Self> {
        match cond_str.to_ascii_lowercase().as_str() {
            "cs" => { Some(BaseStat::Force) }
            "cw" => { Some(BaseStat::Sagesse) }
            "cv" => { Some(BaseStat::Vitalite) }
            "ca" => { Some(BaseStat::Agilite) }
            "ci" => { Some(BaseStat::Intelligence) }
            "cc" => { Some(BaseStat::Chance) }
            "cp" => { Some(BaseStat::PA) }
            "cm" => { Some(BaseStat::PM) }
            _ => None
        }
    }

    pub fn from_dofus_db_val(val: i64) -> BaseStat {
        match val {
            1 => BaseStat::PA,
            23 => BaseStat::PM,
            19 => BaseStat::Po,
            26 => BaseStat::Invo,

            10 => BaseStat::Force,
            13 => BaseStat::Chance,
            14 => BaseStat::Agilite,
            15 => BaseStat::Intelligence,
            25 => BaseStat::Puissance,
            69 => BaseStat::PuissancePiege,
            12 => BaseStat::Sagesse,
            11 => BaseStat::Vitalite,

            16 => BaseStat::DoMulti,
            70 => BaseStat::DoPiege,
            90 => BaseStat::DoEau,
            91 => BaseStat::DoAir,
            89 => BaseStat::DoFeu,
            88 => BaseStat::DoTerre,
            92 => BaseStat::DoNeutre,
            86 => BaseStat::DoCri,
            84 => BaseStat::DoPou,
            125 => BaseStat::DoPerMelee,
            120 => BaseStat::DoPerDist,
            122 => BaseStat::DoPerArme,
            123 => BaseStat::DoPerSo,
            255 => BaseStat::DoPerFinaux, // seems like its not a stat

            18 => BaseStat::Critique,

            79 => BaseStat::Tacle,
            78 => BaseStat::Fuite,
            27 => BaseStat::EsqPA,
            28 => BaseStat::EsqPM,
            82 => BaseStat::RetPA,
            83 => BaseStat::RetPM,

            35 => BaseStat::RePerEau,
            37 => BaseStat::RePerNeutre,
            34 => BaseStat::RePerFeu,
            36 => BaseStat::RePerAir,
            33 => BaseStat::RePerTerre,

            56 => BaseStat::ReFixEau,
            58 => BaseStat::ReFixNeutre,
            55 => BaseStat::ReFixFeu,
            57 => BaseStat::ReFixAir,
            54 => BaseStat::ReFixTerre,
            87 => BaseStat::ReCri,
            85 => BaseStat::RePou,

            121 => BaseStat::RePerDist,
            124 => BaseStat::RePerMelee,
            141 => BaseStat::RePerSo,
            142 => BaseStat::RePerArme,

            49 => BaseStat::Soin,
            44 => BaseStat::Initiative,
            48 => BaseStat::Prospection,
            50 => BaseStat::RenvoiDo,

            29 => BaseStat::Carac29, // idk what that is but some item has it

            _ => BaseStat::Unknown
        }
    }

    pub fn into_dofus_db_val(self) -> i64 {
        match self {
            BaseStat::PA => 1,
            BaseStat::PM => 23,
            BaseStat::Po => 19,
            BaseStat::Invo => 26,

            BaseStat::Force => 10,
            BaseStat::Chance => 13,
            BaseStat::Agilite => 14,
            BaseStat::Intelligence => 15,
            BaseStat::Puissance => 25,
            BaseStat::PuissancePiege => 69,
            BaseStat::Sagesse => 12,
            BaseStat::Vitalite => 11,

            BaseStat::DoMulti => 16,
            BaseStat::DoPiege => 70,
            BaseStat::DoEau => 90,
            BaseStat::DoAir => 91,
            BaseStat::DoFeu => 89,
            BaseStat::DoTerre => 88,
            BaseStat::DoNeutre => 92,
            BaseStat::DoCri => 86,
            BaseStat::DoPou => 84,
            BaseStat::DoPerMelee => 125,
            BaseStat::DoPerDist => 120,
            BaseStat::DoPerArme => 122,
            BaseStat::DoPerSo => 123,
            BaseStat::DoPerFinaux => 255, // seems like its not a stat

            BaseStat::Critique => 18,

            BaseStat::Tacle => 79,
            BaseStat::Fuite => 78,
            BaseStat::EsqPA => 27,
            BaseStat::EsqPM => 28,
            BaseStat::RetPA => 82,
            BaseStat::RetPM => 83,

            BaseStat::RePerEau => 35,
            BaseStat::RePerNeutre => 37,
            BaseStat::RePerFeu => 34,
            BaseStat::RePerAir => 36,
            BaseStat::RePerTerre => 33,

            BaseStat::ReFixEau => 56,
            BaseStat::ReFixNeutre => 58,
            BaseStat::ReFixFeu => 55,
            BaseStat::ReFixAir => 57,
            BaseStat::ReFixTerre => 54,
            BaseStat::ReCri => 87,
            BaseStat::RePou => 85,

            BaseStat::RePerDist => 121,
            BaseStat::RePerMelee => 124,
            BaseStat::RePerSo => 141,
            BaseStat::RePerArme => 142,

            BaseStat::Soin => 49,
            BaseStat::Initiative => 44,
            BaseStat::Prospection => 48,
            BaseStat::RenvoiDo => 50,

            BaseStat::Carac29 => 29, // idk what that is but some item has it

            BaseStat::Unknown => 254,
            BaseStat::BrutaliteRetenue => 200,
            BaseStat::BrutaliteSevere => 201,
        }
    }
}