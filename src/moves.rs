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
    power: u32,
    split: Split,
    accuracy: u8,
    max_pp: u8,
    priority: i8,
    contact: bool,
    effect_list: Vec<MoveEffect>,
}
impl MoveBase {
    pub fn new(
        name: String,
        move_type: Type,
        power: u32,
        split: Split,
        accuracy: u8,
        max_pp: u8,
        priority: i8,
        contact: bool,
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
            effect_list,
        }
    }
    pub fn get_power(&self) -> u32 {
        self.power
    }
    pub fn get_accuracy(&self) -> u8 {
        self.accuracy
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_priority(&self) -> i8 {
        self.priority
    }
    pub fn get_split(&self) -> Split {
        self.split
    }
    pub fn get_max_pp(&self) -> u8 {
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
    chance: u8,
    target: Target,
}
impl MoveEffect {
    pub fn new(effect: Effect, chance: u8, target: Target) -> Self {
        Self {
            effect,
            chance,
            target,
        }
    }
    pub fn get_effect(&self) -> Effect {
        self.effect
    }
    pub fn get_effect_chance(&self) -> u8 {
        self.chance
    }
    pub fn get_target(&self) -> Target {
        self.target
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub move_id: usize,
    pub pp: u8,
}
impl Move {
    pub fn new(move_id: usize) -> Self {
        let mut m = Self { move_id, pp: 0 };
        m.pp = ALL_MOVES_VEC[move_id].get_max_pp();
        m
    }
    pub fn lose_pp(&mut self, i: u8) {
        self.pp = self.pp.saturating_sub(i);
    }
    pub fn get_pp(&self) -> u8 {
        self.pp
    }
    pub fn get_max_pp(&self) -> u8 {
        ALL_MOVES_VEC[self.move_id].get_max_pp()
    }
    pub fn get_name(&self) -> &str {
        ALL_MOVES_VEC[self.move_id].get_name()
    }
    pub fn get_power(&self) -> u32 {
        ALL_MOVES_VEC[self.move_id].get_power()
    }
    pub fn get_priority(&self) -> i8 {
        ALL_MOVES_VEC[self.move_id].get_priority()
    }
    pub fn get_split(&self) -> Split {
        ALL_MOVES_VEC[self.move_id].get_split()
    }
    pub fn get_accuracy(&self) -> u8 {
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
    pub fn has_recoil_hp(&self) -> u8 {
        for i in ALL_MOVES_VEC[self.move_id].get_effects() {
            if let Effect::RecoilHp { fraction } = i.get_effect() {
                return fraction;
            }
        }
        0
    }
    pub fn has_recoil_move(&self) -> u8 {
        for i in ALL_MOVES_VEC[self.move_id].get_effects() {
            if let Effect::RecoilMove { fraction } = i.get_effect() {
                return fraction;
            }
        }
        0
    }
    pub fn has_heal_move(&self) -> u8 {
        for i in ALL_MOVES_VEC[self.move_id].get_effects() {
            if let Effect::HealMove { fraction } = i.get_effect() {
                return fraction;
            }
        }
        0
    }
}
