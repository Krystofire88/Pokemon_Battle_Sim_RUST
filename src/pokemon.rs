use crate::enums::*;
use crate::item::Item;
use crate::species::Species;
use rand::Rng;

#[derive(Clone)]
pub struct Pokemon {
    pub species: Species,
    pub nickname: String,
    pub gender: Gender,
    pub level: i32,
    pub max_hp: i32,
    pub hp: i32,
    pub ability: i32,
    pub non_volitile_status: Status,
    pub volitile_status: Vec<StatusVol>,
    pub hp_iv: i32,
    pub hp_ev: i32,
    pub atk_iv: i32,
    pub atk_ev: i32,
    pub def_iv: i32,
    pub def_ev: i32,
    pub spa_iv: i32,
    pub spa_ev: i32,
    pub spd_iv: i32,
    pub spd_ev: i32,
    pub spe_iv: i32,
    pub spe_ev: i32,
    pub atk_mod: i32,
    pub def_mod: i32,
    pub spa_mod: i32,
    pub spd_mod: i32,
    pub spe_mod: i32,
    pub acc_mod: i32,
    pub eva_mod: i32,
    pub nature: Nature,
    pub held_item: Item,
    pub gmax: bool,
    pub is_dmax: bool,
    pub dmax_level: i32,
    pub tera_type: Type,
    pub terasallized: bool,
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
    fn calc_hp(&self) -> i32 {
        //magic numbers from offical formula
        let mut hp_evs: f64 = self.hp_ev as f64 / 4.0;
        hp_evs = hp_evs.floor();
        let numerator_left: f64 = 2.0 * self.species.get_hp() as f64 + self.hp_iv as f64 + hp_evs;
        let numerator: f64 = numerator_left * self.level as f64;
        let floor: f64 = numerator / 100.0;
        return floor.floor() as i32 + self.level + 10;
    }
}
