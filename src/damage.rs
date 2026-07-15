use crate::active_pkmn::ActivePokemon;
use crate::enums::*;
use crate::helper::*;
use crate::moves::*;
use crate::pokemon::Pokemon;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct DamageModifiers {
    random: f64,
    type_eff: f64,
    weather: f64,
    terrain: f64,
    stab: f64,
    crit: f64,
    other: f64,
}
impl DamageModifiers {
    pub fn new(
        random: f64,
        type_eff: f64,
        weather: f64,
        terrain: f64,
        stab: f64,
        crit: f64,
        other: f64,
    ) -> Self {
        Self {
            random,
            type_eff,
            weather,
            terrain,
            stab,
            crit,
            other,
        }
    }
    pub fn get_random_mod(&self) -> f64 {
        self.random
    }
    pub fn get_type_mod(&self) -> f64 {
        self.type_eff
    }
    pub fn get_weather_mod(&self) -> f64 {
        self.weather
    }
    pub fn get_terrain_mod(&self) -> f64 {
        self.terrain
    }
    pub fn get_stab_mod(&self) -> f64 {
        self.stab
    }
    pub fn get_crit_mod(&self) -> f64 {
        self.crit
    }
    pub fn get_other_mod(&self) -> f64 {
        self.other
    }
}

pub fn damage_calc(
    rng: &mut ThreadRng,
    pokemon_atk: &Pokemon,
    pokemon_def: &Pokemon,
    active_pokemon_atk: &ActivePokemon,
    active_pokemon_def: &ActivePokemon,
    mv: &Move,
    weather: Weather,
    terrain: Terrain,
) -> i32 {
    let level = pokemon_atk.get_level();
    let power = mv.get_power();
    let type_move = mv.get_type();
    let type_atk_1 = pokemon_atk.get_type_1();
    let type_atk_2 = pokemon_atk.get_type_2();
    let type_def_1 = pokemon_def.get_type_1();
    let type_def_2 = pokemon_def.get_type_2();

    let random_modifier: f64 = rng.gen_range(0.85..=1.0);

    let effectiveness_type1: f64 = matchup(type_move, type_def_1);
    let effectiveness_type2: f64 = matchup(type_move, type_def_2);
    let type_modifier = effectiveness_type1 * effectiveness_type2;

    let weather_modifier = calc_weather(weather, type_move);

    let terrain_modifier = calc_terrain(terrain, type_move);

    let mut stab: f64 = 1.0;
    if (type_move == type_atk_1 || type_move == type_atk_2) && type_move != Type::None {
        stab = 1.5;
    }

    let (atk, def, crit) = calc_atk_def(
        rng,
        pokemon_atk,
        pokemon_def,
        active_pokemon_atk,
        active_pokemon_def,
        mv,
    );

    damage(
        atk,
        def,
        level,
        power,
        DamageModifiers::new(
            random_modifier,
            type_modifier,
            weather_modifier,
            terrain_modifier,
            stab,
            crit,
            1.0,
        ),
        pokemon_def.get_hp(),
    )
}
pub fn damage(
    atk: i32,
    def: i32,
    level: i32,
    power: i32,
    mods: DamageModifiers,
    max_damage: i32,
) -> i32 {
    //magic numbers from official formula
    let top_left_bracket = (2 * level) / 5 + 2;
    let atk_over_def: f64 = atk as f64 / def as f64;
    let numerator: f64 = top_left_bracket as f64 * power as f64 * atk_over_def;
    let damage_pre_mod = numerator / 50.0 + 2.0;

    let damage = damage_pre_mod.floor()
        * mods.get_random_mod()
        * mods.get_type_mod()
        * mods.get_weather_mod()
        * mods.get_terrain_mod()
        * mods.get_stab_mod()
        * mods.get_crit_mod()
        * mods.get_other_mod();

    let mut final_damage = damage.round() as i32;

    if final_damage == 0 && mods.get_type_mod() > 0.0 {
        return 1;
    }
    if max_damage < final_damage {
        final_damage = max_damage;
    }

    final_damage
}
fn calc_atk_def(
    rng: &mut ThreadRng,
    pokemon_atk: &Pokemon,
    pokemon_def: &Pokemon,
    active_pokemon_atk: &ActivePokemon,
    active_pokemon_def: &ActivePokemon,
    mv: &Move,
) -> (i32, i32, f64) {
    let mut crit: f64 = 1.0;
    let crit_chance = match active_pokemon_atk.get_crit_stage() {
        0 => 24,
        1 => 8,
        2 => 2,
        _ => 1,
    };

    if 1 == rng.gen_range(1..=crit_chance) {
        crit = 1.5;
    }

    let atk_base;
    let def_base;
    let atk_mod;
    let def_mod;

    if mv.get_split() == Split::Physical {
        atk_base = pokemon_atk.get_atk();
        def_base = pokemon_def.get_def();
        atk_mod = active_pokemon_atk.get_atk();
        def_mod = active_pokemon_def.get_def();
    } else {
        atk_base = pokemon_atk.get_spa();
        def_base = pokemon_def.get_spd();
        atk_mod = active_pokemon_atk.get_spa();
        def_mod = active_pokemon_def.get_spd();
    }

    crit_modifer_rules(atk_base, def_base, atk_mod, def_mod, crit)
}
fn crit_modifer_rules(
    atk_base: i32,
    def_base: i32,
    atk_mod: i32,
    def_mod: i32,
    crit: f64,
) -> (i32, i32, f64) {
    if crit > 1.0 {
        let atk_pos_mod = if atk_mod > 0 { get_mod(atk_mod) } else { 1.0 };
        let def_neg_mod = if def_mod < 0 { get_mod(def_mod) } else { 1.0 };

        let atk = (atk_base as f64 * atk_pos_mod) as i32;
        let def = (def_base as f64 * def_neg_mod) as i32;
        (atk, def, crit)
    } else {
        let atk = (atk_base as f64 * get_mod(atk_mod)) as i32;
        let def = (def_base as f64 * get_mod(def_mod)) as i32;
        (atk, def, crit)
    }
}
fn calc_weather(weather: Weather, type_move: Type) -> f64 {
    match (type_move, weather) {
        (Type::Fire, Weather::Sun | Weather::HarshSun)
        | (Type::Water, Weather::Rain | Weather::HeavyRain) => 1.5,
        _ => 1.0,
    }
}
fn calc_terrain(terrain: Terrain, type_move: Type) -> f64 {
    match (type_move, terrain) {
        (Type::Electric, Terrain::Electric)
        | (Type::Grass, Terrain::Grassy)
        | (Type::Psychic, Terrain::Psychic) => 1.3,
        (Type::Dragon, Terrain::Misty) => 0.5,
        _ => 1.0,
    }
}
