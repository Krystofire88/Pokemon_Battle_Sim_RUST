#![allow(clippy::too_many_arguments)]
use crate::consts::*;
use crate::enums::*;
use crate::move_effects::*;
use rand::Rng;
use rand::rngs::ThreadRng;
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
    pub fn get_name(&self) -> &str {
        &self.name
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
    pub fn get_effects(&self) -> &Vec<MoveEffect> {
        &self.effect_list
    }
}

#[derive(Clone, Copy, Deserialize)]
pub struct MoveEffect {
    effect: Effect,
    chance: i32,
    target: Target,
}
impl MoveEffect {
    pub fn new(effect: Effect, chance: i32, target: Target) -> Self {
        Self {
            effect,
            chance,
            target,
        }
    }
    pub fn get_effect(&self) -> Effect {
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
        m
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
    pub fn get_max_pp(&self) -> i32 {
        ALL_MOVES_VEC[self.move_id].get_max_pp()
    }
    pub fn get_name(&self) -> &str {
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
    pub fn get_effects(&self) -> &Vec<MoveEffect> {
        ALL_MOVES_VEC[self.move_id].get_effects()
    }
    pub fn get_hits(&self, rng: &mut ThreadRng) -> usize {
        for i in ALL_MOVES_VEC[self.move_id].get_effects() {
            if let Effect::MultiHit { min, max } = i.get_effect() {
                if min == max {
                    return min as usize;
                }
                let flip = rng.gen_range(1..=100);
                if flip < 16 {
                    return 5;
                } else if flip < 31 {
                    return 4;
                } else if flip < 66 {
                    return 3;
                } else {
                    return 2;
                }
            }
        }
        1
    }
    pub fn has_recoil_hp(&self) -> i32 {
        for i in ALL_MOVES_VEC[self.move_id].get_effects() {
            if let Effect::Recoil_Hp { fraction } = i.get_effect() {
                return fraction;
            }
        }
        0
    }
    pub fn has_recoil_move(&self) -> i32 {
        for i in ALL_MOVES_VEC[self.move_id].get_effects() {
            if let Effect::Recoil_Move { fraction } = i.get_effect() {
                return fraction;
            }
        }
        0
    }
}
