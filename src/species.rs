use crate::enums::*;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Species {
    name: String,
    type_1: Type,
    type_2: Type,
    hp: i32,
    atk: i32,
    def: i32,
    spa: i32,
    spd: i32,
    spe: i32,
    ability_1: String,
    ability_2: String,
    ability_h: String,
    genderless: bool,
    m_to_f_ratio: i32,
    mega: bool,
    gmax: bool,
    weight: f32,
}
impl Species {
    pub fn new(
        name: String,
        type_1: Type,
        type_2: Type,
        hp: i32,
        atk: i32,
        def: i32,
        spa: i32,
        spd: i32,
        spe: i32,
        ability_1: String,
        ability_2: String,
        ability_h: String,
        genderless: bool,
        m_to_f_ratio: i32,
        mega: bool,
        gmax: bool,
        weight: f32,
    ) -> Species {
        Species {
            name: name,
            type_1: type_1,
            type_2: type_2,
            hp: hp,
            atk: atk,
            def: def,
            spa: spa,
            spd: spd,
            spe: spe,
            ability_1: ability_1,
            ability_2: ability_2,
            ability_h: ability_h,
            genderless: genderless,
            m_to_f_ratio: m_to_f_ratio,
            mega: mega,
            gmax: gmax,
            weight: weight,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_hp(&self) -> i32 {
        self.hp
    }
    pub fn get_atk(&self) -> i32 {
        self.atk
    }
    pub fn get_def(&self) -> i32 {
        self.def
    }
    pub fn get_spa(&self) -> i32 {
        self.spa
    }
    pub fn get_spd(&self) -> i32 {
        self.spd
    }
    pub fn get_spe(&self) -> i32 {
        self.spe
    }
    pub fn get_m_to_f_ratio(&self) -> i32 {
        self.m_to_f_ratio
    }
    pub fn get_genderless(&self) -> bool {
        self.genderless
    }
    pub fn get_type_1(&self) -> Type {
        self.type_1
    }
    pub fn get_type_2(&self) -> Type {
        self.type_2
    }
}
