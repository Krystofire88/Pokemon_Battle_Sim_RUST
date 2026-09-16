#![allow(clippy::too_many_arguments)]
use crate::enums::*;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Species {
    name: String,
    type_1: Type,
    type_2: Type,
    hp: u8,
    atk: u8,
    def: u8,
    spa: u8,
    spd: u8,
    spe: u8,
    ability_1: String,
    ability_2: String,
    ability_h: String,
    genderless: bool,
    m_to_f_ratio: u8,
    mega: bool,
    gmax: bool,
    weight: f64,
}
impl Species {
    pub fn new(
        name: String,
        type_1: Type,
        type_2: Type,
        hp: u8,
        atk: u8,
        def: u8,
        spa: u8,
        spd: u8,
        spe: u8,
        ability_1: String,
        ability_2: String,
        ability_h: String,
        genderless: bool,
        m_to_f_ratio: u8,
        mega: bool,
        gmax: bool,
        weight: f64,
    ) -> Species {
        Species {
            name,
            type_1,
            type_2,
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
            ability_1,
            ability_2,
            ability_h,
            genderless,
            m_to_f_ratio,
            mega,
            gmax,
            weight,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_hp(&self) -> u8 {
        self.hp
    }
    pub fn get_atk(&self) -> u8 {
        self.atk
    }
    pub fn get_def(&self) -> u8 {
        self.def
    }
    pub fn get_spa(&self) -> u8 {
        self.spa
    }
    pub fn get_spd(&self) -> u8 {
        self.spd
    }
    pub fn get_spe(&self) -> u8 {
        self.spe
    }
    pub fn get_m_to_f_ratio(&self) -> u8 {
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
