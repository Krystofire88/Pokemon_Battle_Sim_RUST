use crate::enums::*;
use crate::move_effects::MoveEffects;

#[derive(Clone)]
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
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Clone)]
pub struct MoveEffect {
    effect: MoveEffects,
    chance: i32,
}
impl MoveEffect {
    pub fn new(effect: MoveEffects, chance: i32) -> Self {
        Self { effect, chance }
    }
}

#[derive(Clone)]
pub struct Move {
    pub move_base: MoveBase,
    pp: i32,
}
impl Move {
    pub fn new(move_base: MoveBase) -> Self {
        let mut m = Self { move_base, pp: 0 };
        m.pp = m.move_base.max_pp;
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
}
