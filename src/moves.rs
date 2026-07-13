use crate::consts::*;
use crate::enums::*;
use crate::move_effects::MoveEffects;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct MoveBase {
    name: String,
    move_type: Type,
    power: i32,
    split: Split,
    accuracy: i32,
    max_pp: i32,
    priority: i32,
    contact: bool,
    protect: bool,
    effect_list: Vec<MoveEffect>,
}
impl MoveBase {
    pub fn new(
        name: String,
        move_type: Type,
        power: i32,
        split: Split,
        accuracy: i32,
        max_pp: i32,
        priority: i32,
        contact: bool,
        protect: bool,
        effect_list: Vec<MoveEffect>,
    ) -> Self {
        Self {
            name,
            move_type,
            power,
            split,
            accuracy,
            max_pp,
            priority,
            contact,
            protect,
            effect_list,
        }
    }
    pub fn get_power(&self) -> i32 {
        self.power
    }
    pub fn get_accuracy(&self) -> i32 {
        self.accuracy
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone, Deserialize)]
pub struct MoveEffect {
    effect: MoveEffects,
    chance: i32,
}
impl MoveEffect {
    pub fn new(effect: MoveEffects, chance: i32) -> Self {
        Self { effect, chance }
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub move_id: i32,
    pub pp: i32,
}
impl Move {
    pub fn new(move_id: i32) -> Self {
        let mut m = Self { move_id, pp: 0 };
        m.pp = ALL_MOVES_VEC[move_id as usize].max_pp;
        return m;
    }
    pub fn lose_pp(&mut self, i: i32) {
        self.pp -= i;
        if self.pp < 0 {
            self.pp = 0;
        }
    }
    pub fn get_pp(&self) -> i32 {
        self.pp
    }
    pub fn get_name(&self) -> String {
        ALL_MOVES_VEC[self.move_id as usize].get_name()
    }
    pub fn get_power(&self) -> i32 {
        ALL_MOVES_VEC[self.move_id as usize].get_power()
    }
}
