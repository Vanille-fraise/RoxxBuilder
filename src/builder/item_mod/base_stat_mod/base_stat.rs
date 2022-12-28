use std::cmp::max;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use num_derive::FromPrimitive;
use strum_macros::EnumIter;

extern crate num;

#[repr(u8)]
#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize, Debug, FromPrimitive, EnumIter, PartialOrd, Ord)]
pub enum BaseStat {
    // my_item[effects][i][characteristic]
    PA = 1,
    PM = 23,
    Po = 19,
    Invo = 26,

    Force = 10,
    Chance = 13,
    Agilite = 14,
    Intelligence = 15,
    Puissance = 25,
    PuissancePiege = 69,
    Sagesse = 12,
    Vitalite = 11,

    DoMulti = 16,
    DoPiege = 70,
    DoEau = 90,
    DoAir = 91,
    DoFeu = 89,
    DoTerre = 88,
    DoNeutre = 92,
    DoCri = 86,
    DoPou = 84,
    DoPerMelee = 125,
    DoPerDist = 120,
    DoPerArme = 122,
    DoPerSo = 123,
    DoPerFinaux = 255, // seems like its not a stat

    Critique = 18,

    Tacle = 79,
    Fuite = 78,
    EsqPA = 27,
    EsqPM = 28,
    RetPA = 82,
    RetPM = 83,

    RePerEau = 35,
    RePerNeutre = 37,
    RePerFeu = 34,
    RePerAir = 36,
    RePerTerre = 33,

    ReFixEau = 56,
    ReFixNeutre = 58,
    ReFixFeu = 55,
    ReFixAir = 57,
    ReFixTerre = 54,
    ReCri = 87,
    RePou = 85,

    RePerDist = 121,
    RePerMelee = 124,
    RePerSo = 141,
    RePerArme = 142,

    Soin = 49,
    Initiative = 44,
    Prospection = 48,
    RenvoiDo = 50,

    Carac29 = 29, // idk what that is but some item has it

    Unknown = 254,
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

    pub fn from_effects_json_value(value: &serde_json::Value) -> HashMap<BaseStat, i64> {
        let m: HashMap<BaseStat, i64> = value.as_array().unwrap_or(&mut vec![]).iter().filter(|v| { v["characteristic"].as_i64().unwrap_or(-1) > 0 }).map(|v| {
            let stat: BaseStat = num::FromPrimitive::from_i64(v["characteristic"].as_i64().unwrap()).unwrap_or(BaseStat::Unknown);
            let to = v["to"].as_i64().unwrap_or(0);
            let from = v["from"].as_i64().unwrap_or(0);
            (stat, max(to, from))
        }).collect();
        m
    }
}