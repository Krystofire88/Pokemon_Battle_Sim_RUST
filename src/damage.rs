use crate::active_pkmn::ActivePokemon;
use crate::enums::Split::Physical;
use crate::enums::*;
use crate::field::*;
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
    screen_mod: f64,
    burn: f64,
}
impl DamageModifiers {
    pub fn new(
        random: f64,
        type_eff: f64,
        weather: f64,
        terrain: f64,
        stab: f64,
        crit: f64,
        screen_mod: f64,
        burn: f64,
    ) -> Self {
        Self {
            random,
            type_eff,
            weather,
            terrain,
            stab,
            crit,
            screen_mod,
            burn,
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
    fn get_screen_mod(&self) -> f64 {
        self.screen_mod
    }
    pub fn get_burn_mod(&self) -> f64 {
        self.burn
    }
    pub fn get_other_mod(&self) -> f64 {
        let mut numerator = 4096.0;
        let denominator = 4096.0;

        if self.get_crit_mod() == 1.0 {
            numerator *= self.get_screen_mod();
        }
        numerator / denominator
    }
}

pub fn damage_calc(
    rng: &mut ThreadRng,
    pokemon_atk: &Pokemon,
    pokemon_def: &Pokemon,
    active_pokemon_atk: &ActivePokemon,
    active_pokemon_def: &ActivePokemon,
    mv: &Move,
    field: &Field,
    field_side: &FieldSide,
) -> u16 {
    let level = pokemon_atk.get_level();
    let power = mv.get_power();
    let type_move = mv.get_type();
    let type_atk_1 = pokemon_atk.get_type_1();
    let type_atk_2 = pokemon_atk.get_type_2();
    let type_def_1 = pokemon_def.get_type_1();
    let type_def_2 = pokemon_def.get_type_2();

    let random_modifier: f64 = rng.gen_range(85..=100) as f64 / 100.0;

    let effectiveness_type1: f64 = matchup(type_move, type_def_1);
    let effectiveness_type2: f64 = matchup(type_move, type_def_2);
    let type_modifier = effectiveness_type1 * effectiveness_type2;

    if type_modifier <= 0.0 {
        return 0;
    }

    let burn = if pokemon_atk.get_status() == Status::Burn && mv.get_split() == Split::Physical {
        0.5
    } else {
        1.0
    };

    let weather_modifier = calc_weather(field.get_weather(), type_move);

    let terrain_modifier = calc_terrain(field.get_terrain(), type_move);

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

    let screen_modifier = calc_screens(mv, field_side);

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
            screen_modifier,
            burn,
        ),
    )
}
pub fn damage(atk: u32, def: u32, level: u8, base_power: u32, mods: DamageModifiers) -> u16 {
    //magic numbers from official formula
    let top_left_bracket = ((2.0 * level as f64) / 5.0).floor() + 2.0;
    let atk_over_def: f64 = atk as f64 / def as f64;
    let power = base_power as f64 * mods.get_terrain_mod();
    let numerator: f64 = top_left_bracket * power * atk_over_def;
    let mut damage = (numerator.floor() / 50.0).floor() + 2.0;

    for m in [
        //target
        //parental bond
        mods.get_weather_mod(),
        //glaive rush
        mods.get_crit_mod(),
        mods.get_random_mod(),
        mods.get_stab_mod(),
        mods.get_type_mod(),
        mods.get_burn_mod(),
        mods.get_other_mod(),
    ] {
        damage = poke_round(damage * m);
    }

    damage.max(1.0) as u16
}
fn calc_atk_def(
    rng: &mut ThreadRng,
    pokemon_atk: &Pokemon,
    pokemon_def: &Pokemon,
    active_pokemon_atk: &ActivePokemon,
    active_pokemon_def: &ActivePokemon,
    mv: &Move,
) -> (u32, u32, f64) {
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

    let (atk_base, def_base, atk_mod, def_mod) = if mv.get_split() == Split::Physical {
        (
            pokemon_atk.get_atk(),
            pokemon_def.get_def(),
            active_pokemon_atk.get_atk(),
            active_pokemon_def.get_def(),
        )
    } else {
        (
            pokemon_atk.get_spa(),
            pokemon_def.get_spd(),
            active_pokemon_atk.get_spa(),
            active_pokemon_def.get_spd(),
        )
    };

    crit_modifer_rules(atk_base, def_base, atk_mod, def_mod, crit)
}
fn crit_modifer_rules(
    atk_base: u32,
    def_base: u32,
    atk_mod: i8,
    def_mod: i8,
    crit: f64,
) -> (u32, u32, f64) {
    if crit > 1.0 {
        let atk_pos_mod = if atk_mod > 0 { get_mod(atk_mod) } else { 1.0 };
        let def_neg_mod = if def_mod < 0 { get_mod(def_mod) } else { 1.0 };

        let atk = (atk_base as f64 * atk_pos_mod) as u32;
        let def = (def_base as f64 * def_neg_mod) as u32;
        (atk, def, crit)
    } else {
        let atk = (atk_base as f64 * get_mod(atk_mod)) as u32;
        let def = (def_base as f64 * get_mod(def_mod)) as u32;
        (atk, def, crit)
    }
}
fn poke_round(num: f64) -> f64 {
    if num.fract() > 0.5 {
        num.ceil()
    } else {
        num.floor()
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
    //add grassy terain halving of bulldoze, eq and magnitude
    match (type_move, terrain) {
        (Type::Electric, Terrain::Electric)
        | (Type::Grass, Terrain::Grassy)
        | (Type::Psychic, Terrain::Psychic) => 1.3,
        (Type::Dragon, Terrain::Misty) => 0.5,
        _ => 1.0,
    }
}
fn calc_screens(mv: &Move, field_side: &FieldSide) -> f64 {
    match (mv.get_split(), field_side.is_aurora_veil()) {
        (_, true) => 0.5,
        (Split::Physical, _) => {
            if field_side.is_reflect() {
                0.5
            } else {
                1.0
            }
        }
        (Split::Special, _) => {
            if field_side.is_light_screen() {
                0.5
            } else {
                1.0
            }
        }
        (_, _) => 1.0,
    }
}
/* this is for doubles
fn calc_screens(mv: &Move, field_side: &FieldSide) -> f64 {
    match (mv.get_split(), field_side.is_aurora_veil()) {
        (_, true) => 2732.0 / 4096.0,
        (Split::Physical, _) => {
            if field_side.is_reflect() {
                2732.0 / 4096.0
            } else {
                1.0
            }
        }
        (Split::Special, _) => {
            if field_side.is_light_screen() {
                2732.0 / 4096.0
            } else {
                1.0
            }
        }
        (_, _) => 1.0,
    }
}
*/
