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
