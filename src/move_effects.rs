use crate::enums::*;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub enum Effect {
    ChangeStat { stat: Stat, stages: i8 },

    InflictStatus { status: Status },

    InflictStatusVol { status: StatusVol },

    Protect,

    MultiHit { min: u8, max: u8 },

    HealHp { fraction: u8 },

    HealMove { fraction: u8 },

    RecoilHp { fraction: u8 },

    RecoilMove { fraction: u8 },

    Weather { weather: Weather },

    Terrain { terrain: Terrain },
}

#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum Target {
    User,
    Opponent,
    All,
}
