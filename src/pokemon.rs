use crate::enums::*;
use crate::helper::*;
use crate::item::Item;
use crate::moves::Move;
use crate::species::Species;
use rand::Rng;

#[derive(Clone)]
pub struct Pokemon {
    pub species: Species,
    nickname: String,
    gender: Gender,
    level: i32,
    max_hp: i32,
    hp: i32,
    ability: i32,
    non_volitile_status: Status,
    volitile_status: Vec<StatusVol>,
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
    atk_mod: i32,
    def_mod: i32,
    spa_mod: i32,
    spd_mod: i32,
    spe_mod: i32,
    acc_mod: i32,
    eva_mod: i32,
    nature: Nature,
    held_item: Item,
    gmax: bool,
    is_dmax: bool,
    dmax_level: i32,
    tera_type: Type,
    terasallized: bool,
    move_set: [Move; 4],
}
impl Pokemon {
    pub fn new(
        species: Species,
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
        move_set: [Move; 4],
    ) -> Self {
        let mut pk = Self {
            species,
            nickname,
            gender,
            level,
            max_hp: 0,
            hp: 0,
            ability,
            non_volitile_status: Status::None,
            volitile_status: Vec::new(),
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
            atk_mod: 0,
            def_mod: 0,
            spa_mod: 0,
            spd_mod: 0,
            spe_mod: 0,
            acc_mod: 0,
            eva_mod: 0,
            nature,
            held_item,
            gmax,
            is_dmax: false,
            dmax_level,
            tera_type,
            terasallized: false,
            move_set,
        };

        pk.max_hp = pk.calc_hp();
        pk.hp = pk.max_hp;

        return pk;
    }
    pub fn new_easy(species: Species, level: i32) -> Self {
        let mut pk = Self {
            species,
            nickname: "Placeholder".to_string(),
            gender: Gender::Genderless,
            level,
            max_hp: 0,
            hp: 0,
            ability: rand::thread_rng().gen_range(0..=3),
            non_volitile_status: Status::None,
            volitile_status: Vec::new(),
            hp_iv: rand::thread_rng().gen_range(0..=31),
            hp_ev: rand::thread_rng().gen_range(0..=88),
            atk_iv: rand::thread_rng().gen_range(0..=31),
            atk_ev: rand::thread_rng().gen_range(0..=88),
            def_iv: rand::thread_rng().gen_range(0..=31),
            def_ev: rand::thread_rng().gen_range(0..=88),
            spa_iv: rand::thread_rng().gen_range(0..=31),
            spa_ev: rand::thread_rng().gen_range(0..=88),
            spd_iv: rand::thread_rng().gen_range(0..=31),
            spd_ev: rand::thread_rng().gen_range(0..=88),
            spe_iv: rand::thread_rng().gen_range(0..=31),
            spe_ev: rand::thread_rng().gen_range(0..=88),
            atk_mod: 0,
            def_mod: 0,
            spa_mod: 0,
            spd_mod: 0,
            spe_mod: 0,
            acc_mod: 0,
            eva_mod: 0,
            nature: Nature::Serious,
            held_item: Item {},
            gmax: false,
            is_dmax: false,
            dmax_level: 10,
            tera_type: Type::Normal,
            terasallized: false,
            move_set: [Move; 4],
        };

        pk.max_hp = pk.calc_hp();
        pk.hp = pk.max_hp;

        if !pk.species.get_genderless() {
            let flip = rand::thread_rng().gen_range(0..100);
            if flip > pk.species.get_m_to_f_ratio() {
                pk.gender = Gender::Female;
            } else {
                pk.gender = Gender::Male;
            }
        }

        return pk;
    }
    fn calc_stat(&self, base: i32, iv: i32, ev: i32) -> i32 {
        let mut evs: f64 = ev as f64 / 4.0;
        evs = evs.floor();
        let numerator_left: f64 = 2.0 * base as f64 + iv as f64 + evs;
        let numerator: f64 = numerator_left * self.level as f64;
        let floor: f64 = numerator / 100.0;
        return floor.floor() as i32;
    }
    fn calc_hp(&self) -> i32 {
        let base = self.calc_stat(self.species.get_hp(), self.hp_iv, self.hp_ev);
        return base + self.level + 10;
    }
    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }
    pub fn get_level(&self) -> i32 {
        self.level
    }
    pub fn get_hp(&self) -> i32 {
        self.hp
    }
    pub fn get_atk(&self) -> i32 {
        let base = self.calc_stat(self.species.get_atk(), self.atk_iv, self.atk_ev);

        let nature = match self.nature {
            Nature::Lonely | Nature::Adamant | Nature::Naughty | Nature::Brave => 1.1,
            Nature::Bold | Nature::Modest | Nature::Calm | Nature::Timid => 0.9,
            _ => 1.0,
        };

        let atk_stat = (base as f64 + 5.0) * nature;

        let atk: f64 = atk_stat.floor() * get_mod(self.atk_mod);
        return atk.floor() as i32;
    }
    pub fn get_def(&self) -> i32 {
        let base = self.calc_stat(self.species.get_def(), self.def_iv, self.def_ev);

        let nature = match self.nature {
            Nature::Bold | Nature::Impish | Nature::Lax | Nature::Relaxed => 1.1,
            Nature::Lonely | Nature::Mild | Nature::Gentle | Nature::Hasty => 0.9,
            _ => 1.0,
        };

        let def_stat = (base as f64 + 5.0) * nature;

        let def = def_stat.floor() * get_mod(self.def_mod);
        def.floor() as i32
    }

    pub fn get_spa(&self) -> i32 {
        let base = self.calc_stat(self.species.get_spa(), self.spa_iv, self.spa_ev);

        let nature = match self.nature {
            Nature::Modest | Nature::Mild | Nature::Rash | Nature::Quiet => 1.1,
            Nature::Adamant | Nature::Impish | Nature::Careful | Nature::Jolly => 0.9,
            _ => 1.0,
        };

        let spa_stat = (base as f64 + 5.0) * nature;

        let spa = spa_stat.floor() * get_mod(self.spa_mod);
        spa.floor() as i32
    }

    pub fn get_spd(&self) -> i32 {
        let base = self.calc_stat(self.species.get_spd(), self.spd_iv, self.spd_ev);

        let nature = match self.nature {
            Nature::Calm | Nature::Gentle | Nature::Careful | Nature::Sassy => 1.1,
            Nature::Naughty | Nature::Lax | Nature::Rash | Nature::Naive => 0.9,
            _ => 1.0,
        };

        let spd_stat = (base as f64 + 5.0) * nature;

        let spd = spd_stat.floor() * get_mod(self.spd_mod);
        spd.floor() as i32
    }

    pub fn get_spe(&self) -> i32 {
        let base = self.calc_stat(self.species.get_spe(), self.spe_iv, self.spe_ev);

        let nature = match self.nature {
            Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive => 1.1,
            Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy => 0.9,
            _ => 1.0,
        };

        let spe_stat = (base as f64 + 5.0) * nature;

        let spe = spe_stat.floor() * get_mod(self.spe_mod);
        spe.floor() as i32
    }
    pub fn get_acc(&self) -> i32 {
        self.acc_mod
    }
    pub fn get_eva(&self) -> i32 {
        self.eva_mod
    }
}
