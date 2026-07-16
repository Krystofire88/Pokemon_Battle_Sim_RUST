#![allow(clippy::too_many_arguments)]
use crate::consts::*;
use crate::enums::*;
use crate::item::Item;
use crate::moves::Move;
use crate::poke_println;
use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Clone)]
pub struct Pokemon {
    species_id: usize,
    nickname: String,
    gender: Gender,
    level: i32,
    max_hp: i32,
    hp: i32,
    ability: i32,
    non_volatile_status: Status,
    hp_iv: i32,
    hp_ev: i32,
    atk_iv: i32,
    atk_ev: i32,
    def_iv: i32,
    def_ev: i32,
    spa_iv: i32,
    spa_ev: i32,
    spd_iv: i32,
    spd_ev: i32,
    spe_iv: i32,
    spe_ev: i32,
    nature: Nature,
    held_item: Item,
    gmax: bool,
    dmax_level: i32,
    tera_type: Type,
    terastallized: bool,
    pub move_set: Vec<Move>,
}
impl Pokemon {
    pub fn new(
        species_id: usize,
        nickname: String,
        gender: Gender,
        level: i32,
        ability: i32,
        hp_iv: i32,
        hp_ev: i32,
        atk_iv: i32,
        atk_ev: i32,
        def_iv: i32,
        def_ev: i32,
        spa_iv: i32,
        spa_ev: i32,
        spd_iv: i32,
        spd_ev: i32,
        spe_iv: i32,
        spe_ev: i32,
        nature: Nature,
        held_item: Item,
        gmax: bool,
        dmax_level: i32,
        tera_type: Type,
    ) -> Self {
        let mut pk = Self {
            species_id,
            nickname,
            gender,
            level,
            max_hp: 0,
            hp: 0,
            ability,
            non_volatile_status: Status::None,
            hp_iv,
            hp_ev,
            atk_iv,
            atk_ev,
            def_iv,
            def_ev,
            spa_iv,
            spa_ev,
            spd_iv,
            spd_ev,
            spe_iv,
            spe_ev,
            nature,
            held_item,
            gmax,
            dmax_level,
            tera_type,
            terastallized: false,
            move_set: Vec::new(),
        };

        pk.max_hp = pk.calc_hp();
        pk.hp = pk.max_hp;

        pk
    }
    pub fn new_easy(species_id: usize, level: i32, rng: &mut ThreadRng) -> Self {
        let mut pk = Self {
            species_id,
            nickname: "Placeholder".to_string(),
            gender: Gender::Genderless,
            level,
            max_hp: 0,
            hp: 0,
            ability: rng.gen_range(0..=3),
            non_volatile_status: Status::None,
            hp_iv: rng.gen_range(0..=31),
            hp_ev: rng.gen_range(0..=88),
            atk_iv: rng.gen_range(0..=31),
            atk_ev: rng.gen_range(0..=88),
            def_iv: rng.gen_range(0..=31),
            def_ev: rng.gen_range(0..=88),
            spa_iv: rng.gen_range(0..=31),
            spa_ev: rng.gen_range(0..=88),
            spd_iv: rng.gen_range(0..=31),
            spd_ev: rng.gen_range(0..=88),
            spe_iv: rng.gen_range(0..=31),
            spe_ev: rng.gen_range(0..=88),
            nature: Nature::Serious,
            held_item: Item {},
            gmax: false,
            dmax_level: 10,
            tera_type: Type::Normal,
            terastallized: false,
            move_set: Vec::new(),
        };
        pk.nickname = pk.get_species_name().to_string();
        pk.max_hp = pk.calc_hp();
        pk.hp = pk.max_hp;

        if !pk.get_genderless() {
            let flip = rng.gen_range(0..100);
            if flip > pk.get_m_to_f_ratio() {
                pk.gender = Gender::Female;
            } else {
                pk.gender = Gender::Male;
            }
        }
        if pk.get_type_2() == Type::None {
            pk.tera_type = pk.get_type_1();
        } else {
            if rng.gen_bool(0.5) {
                pk.tera_type = pk.get_type_1();
            } else {
                pk.tera_type = pk.get_type_2();
            }
        }

        pk
    }
    pub fn add_move(&mut self, mv: Move) {
        if self.move_set.len() < 4 {
            self.move_set.push(mv);
        }
    }
    pub fn get_info(&self) {
        poke_println!("");
        poke_println!("{}", self.get_nickname());
        poke_println!("{}", self.level);
        poke_println!("{} / {}", self.hp, self.max_hp);
        poke_println!("");
    }
    pub fn heal(&mut self, fraction: i32) {
        self.hp += (self.max_hp as f64 / fraction as f64).floor() as i32;
        if self.hp > self.max_hp {
            self.hp = self.max_hp
        }
    }
    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }
    pub fn take_chip_damage(&mut self, fraction: i32) {
        let damage = (self.max_hp as f64 / fraction as f64).floor() as i32;
        self.take_damage(damage);
    }
    pub fn take_toxic_damage(&mut self, timer: i32) {
        let base_damage = self.max_hp as f64 / 16.0;
        let damage = (base_damage * timer as f64).floor() as i32;
        self.take_damage(damage);
    }
    fn calc_stat(&self, base: i32, iv: i32, ev: i32) -> i32 {
        let mut evs: f64 = ev as f64 / 4.0;
        evs = evs.floor();
        let numerator_left: f64 = 2.0 * base as f64 + iv as f64 + evs;
        let numerator: f64 = numerator_left * self.level as f64;
        let floor: f64 = numerator / 100.0;
        floor.floor() as i32
    }
    fn calc_hp(&self) -> i32 {
        let base = self.calc_stat(self.get_base_hp(), self.hp_iv, self.hp_ev);
        base + self.level + 10
    }
    pub fn get_level(&self) -> i32 {
        self.level
    }
    pub fn get_hp(&self) -> i32 {
        self.hp
    }
    pub fn get_atk(&self) -> i32 {
        let base = self.calc_stat(self.get_base_atk(), self.atk_iv, self.atk_ev);

        let nature = match self.nature {
            Nature::Lonely | Nature::Adamant | Nature::Naughty | Nature::Brave => 1.1,
            Nature::Bold | Nature::Modest | Nature::Calm | Nature::Timid => 0.9,
            _ => 1.0,
        };

        let atk_stat = (base as f64 + 5.0) * nature;

        let atk: f64 = atk_stat.floor();
        atk.floor() as i32
    }
    pub fn get_def(&self) -> i32 {
        let base = self.calc_stat(self.get_base_def(), self.def_iv, self.def_ev);

        let nature = match self.nature {
            Nature::Bold | Nature::Impish | Nature::Lax | Nature::Relaxed => 1.1,
            Nature::Lonely | Nature::Mild | Nature::Gentle | Nature::Hasty => 0.9,
            _ => 1.0,
        };

        let def_stat = (base as f64 + 5.0) * nature;

        let def = def_stat.floor();
        def.floor() as i32
    }

    pub fn get_spa(&self) -> i32 {
        let base = self.calc_stat(self.get_base_spa(), self.spa_iv, self.spa_ev);

        let nature = match self.nature {
            Nature::Modest | Nature::Mild | Nature::Rash | Nature::Quiet => 1.1,
            Nature::Adamant | Nature::Impish | Nature::Careful | Nature::Jolly => 0.9,
            _ => 1.0,
        };

        let spa_stat = (base as f64 + 5.0) * nature;

        let spa = spa_stat.floor();
        spa.floor() as i32
    }

    pub fn get_spd(&self) -> i32 {
        let base = self.calc_stat(self.get_base_spd(), self.spd_iv, self.spd_ev);

        let nature = match self.nature {
            Nature::Calm | Nature::Gentle | Nature::Careful | Nature::Sassy => 1.1,
            Nature::Naughty | Nature::Lax | Nature::Rash | Nature::Naive => 0.9,
            _ => 1.0,
        };

        let spd_stat = (base as f64 + 5.0) * nature;

        let spd = spd_stat.floor();
        spd.floor() as i32
    }

    pub fn get_spe(&self) -> i32 {
        let base = self.calc_stat(self.get_base_spe(), self.spe_iv, self.spe_ev);

        let nature = match self.nature {
            Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive => 1.1,
            Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy => 0.9,
            _ => 1.0,
        };

        let spe_stat = (base as f64 + 5.0) * nature;

        let spe = spe_stat.floor();
        spe.floor() as i32
    }
    pub fn get_nickname(&self) -> &str {
        &self.nickname
    }
    pub fn get_status(&self) -> Status {
        self.non_volatile_status
    }
    pub fn get_status_mut(&mut self) -> &mut Status {
        &mut self.non_volatile_status
    }
    fn get_base_hp(&self) -> i32 {
        ALL_SPECIES_VEC[self.species_id].get_hp()
    }
    fn get_base_atk(&self) -> i32 {
        ALL_SPECIES_VEC[self.species_id].get_atk()
    }
    fn get_base_def(&self) -> i32 {
        ALL_SPECIES_VEC[self.species_id].get_def()
    }
    fn get_base_spa(&self) -> i32 {
        ALL_SPECIES_VEC[self.species_id].get_spa()
    }
    fn get_base_spd(&self) -> i32 {
        ALL_SPECIES_VEC[self.species_id].get_spd()
    }
    fn get_base_spe(&self) -> i32 {
        ALL_SPECIES_VEC[self.species_id].get_spe()
    }
    fn get_genderless(&self) -> bool {
        ALL_SPECIES_VEC[self.species_id].get_genderless()
    }
    fn get_m_to_f_ratio(&self) -> i32 {
        ALL_SPECIES_VEC[self.species_id].get_m_to_f_ratio()
    }
    pub fn get_species_name(&self) -> &str {
        ALL_SPECIES_VEC[self.species_id].get_name()
    }
    pub fn get_type_1(&self) -> Type {
        if self.terastallized {
            self.tera_type
        } else {
            ALL_SPECIES_VEC[self.species_id].get_type_1()
        }
    }
    pub fn get_type_2(&self) -> Type {
        if self.terastallized {
            Type::None
        } else {
            ALL_SPECIES_VEC[self.species_id].get_type_2()
        }
    }
    pub fn inflict_status(&mut self, status: Status) {
        if self.non_volatile_status == Status::None {
            self.non_volatile_status = status
        }
    }
}
