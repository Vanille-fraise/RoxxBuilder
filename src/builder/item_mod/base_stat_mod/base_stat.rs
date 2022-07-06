use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize, Debug)]
pub enum BaseStat {
    PA,
    PM,
    Po,

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
    DoPerMelee,
    DoPerDist,
    DoPerArme,
    DoPerSo,
    DoPerFinaux,

    Critique,

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

    RePerDist,
    RePerMelee,
    RePerSo,
    RePerArme,
}