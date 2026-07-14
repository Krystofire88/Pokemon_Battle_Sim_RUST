use crate::consts::*;
use crate::enums::*;
use crate::move_effects::*;
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
    pub fn get_priority(&self) -> i32 {
        self.priority
    }
    pub fn get_split(&self) -> Split {
        self.split
    }
    pub fn get_max_pp(&self) -> i32 {
        self.max_pp
    }
    pub fn get_type(&self) -> Type {
        self.move_type
    }
    pub fn get_effects(&self) -> Vec<MoveEffect> {
        self.effect_list.clone()
    }
}

#[derive(Clone, Copy, Deserialize)]
pub struct MoveEffect {
    effect: MoveEffects,
    chance: i32,
    target: Target,
}
impl MoveEffect {
    pub fn new(effect: MoveEffects, chance: i32, target: Target) -> Self {
        Self {
            effect,
            chance,
            target,
        }
    }
    pub fn get_effect(&self) -> MoveEffects {
        self.effect
    }
    pub fn get_effect_chance(&self) -> i32 {
        self.chance
    }
    pub fn get_target(&self) -> Target {
        self.target
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub move_id: usize,
    pub pp: i32,
}
impl Move {
    pub fn new(move_id: usize) -> Self {
        let mut m = Self { move_id, pp: 0 };
        m.pp = ALL_MOVES_VEC[move_id].get_max_pp();
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
        ALL_MOVES_VEC[self.move_id].get_name()
    }
    pub fn get_power(&self) -> i32 {
        ALL_MOVES_VEC[self.move_id].get_power()
    }
    pub fn get_priority(&self) -> i32 {
        ALL_MOVES_VEC[self.move_id].get_priority()
    }
    pub fn get_split(&self) -> Split {
        ALL_MOVES_VEC[self.move_id].get_split()
    }
    pub fn get_accuracy(&self) -> i32 {
        ALL_MOVES_VEC[self.move_id].get_accuracy()
    }
    pub fn get_type(&self) -> Type {
        ALL_MOVES_VEC[self.move_id].get_type()
    }
    pub fn get_effects(&self) -> Vec<MoveEffect> {
        ALL_MOVES_VEC[self.move_id].get_effects()
    }
}
