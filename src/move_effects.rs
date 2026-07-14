use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub enum MoveEffects {
    RaiseAtkOnce,
    RaiseAtkTwice,
    RaiseAtkFull,
    DropAtkOnce,
    DropAtkTwice,
    DropAtkFull,
}

#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum Target {
    User,
    Opponent,
    All,
}
