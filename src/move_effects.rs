use crate::enums::*;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub enum Effect {
    ChangeStat {
        stat: Stat,
        stages: i8,
    },

    ChangeAllStats {
        stages: i8,
    },

    InflictStatus {
        status: Status,
    },

    InflictMultiStatus {
        status_1: Status,
        status_2: Status,
        status_3: Status,
    },

    InflictStatusVol {
        status: StatusVol,
    },

    Protect,

    MultiHit {
        min: u8,
        max: u8,
    },

    MultiHitAcc {
        max: u8,
        increment: u8,
    },

    HealStatuses,

    HealHp {
        fraction: u8,
    },

    HealMove {
        fraction: u8,
    },

    RecoilHp {
        fraction: u8,
    },

    RecoilMove {
        fraction: u8,
    },

    Weather {
        weather: Weather,
    },

    Terrain {
        terrain: Terrain,
    },

    FieldSetter {
        effect: FieldEffect,
    },

    FieldSideSetter {
        effect: FieldSideEffect,
    },
}

#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum Target {
    User,
    Opponent,
    All,
}

#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum FieldEffect {
    Gravity,
    TrickRoom,
    WonderRoom,
    MagicRoom,
}

#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum FieldSideEffect {
    StealthRock,
    SharpSteel,
    Spikes,
    ToxicSpikes,
    StickyWeb,
    Reflect,
    LightScreen,
    AuroraVeil,
    Tailwind,
    Mist,
    Safeguard,
}
