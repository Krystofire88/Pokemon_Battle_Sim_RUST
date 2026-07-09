use crate::enums::*;
#[derive(Clone)]
pub struct Species {
    name: String,
    type1: Type,
    type2: Type,
    hp: i32,
    atk: i32,
    def: i32,
    spa: i32,
    spd: i32,
    spe: i32,
    ability1: String,
    ability2: String,
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
        type1: Type,
        type2: Type,
        hp: i32,
        atk: i32,
        def: i32,
        spa: i32,
        spd: i32,
        spe: i32,
        ability1: String,
        ability2: String,
        ability_h: String,
        genderless: bool,
        m_to_f_ratio: i32,
        mega: bool,
        gmax: bool,
        weight: f32,
    ) -> Species {
        Species {
            name: name,
            type1: type1,
            type2: type2,
            hp: hp,
            atk: atk,
            def: def,
            spa: spa,
            spd: spd,
            spe: spe,
            ability1: ability1,
            ability2: ability2,
            ability_h: ability_h,
            genderless: genderless,
            m_to_f_ratio: m_to_f_ratio,
            mega: mega,
            gmax: gmax,
            weight: weight,
        }
    }

    pub fn get_hp(&self) -> i32 {
        self.hp
    }
    pub fn get_m_to_f_ratio(&self) -> i32 {
        self.m_to_f_ratio
    }
    pub fn get_genderless(&self) -> bool {
        self.genderless
    }
}
