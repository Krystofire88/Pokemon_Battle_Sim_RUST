use crate::enums::*;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub enum Effect {
    ChangeStat { stat: Stat, stages: i32 },

    InflictStatus { status: Status },

    InflictStatusVol { status: StatusVol },

    Protect,

    MultiHit { min: i32, max: i32 },

    Heal { fraction: i32 },

    Recoil { fraction: i32 },
}

#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum Target {
    User,
    Opponent,
    All,
}
