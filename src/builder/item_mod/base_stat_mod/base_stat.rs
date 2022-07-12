use serde::{Deserialize, Serialize};

#[repr(u8)]
#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize, Debug)]
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
}