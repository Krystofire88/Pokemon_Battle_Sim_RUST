use crate::enums::*;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub enum Effect {
    ChangeStat { stat: Stat, stages: i32 },

    InflictStatus { status: Status },

    InflictStatusVol { status: StatusVol },

    Protect,

    MultiHit { min: i32, max: i32 },

    HealHp { fraction: i32 },

    HealMove { fraction: i32 },

    RecoilHp { fraction: i32 },

    RecoilMove { fraction: i32 },
}

#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum Target {
    User,
    Opponent,
    All,
}
