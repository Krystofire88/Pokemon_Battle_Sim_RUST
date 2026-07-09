mod enums;
mod field;
mod helper;
mod item;
mod moves;
mod pokemon;
mod species;

use enums::*;
use helper::*;
use item::Item;
use moves::MoveBase;
use pokemon::Pokemon;
use rand::Rng;
use species::Species;

fn calc_damage(atk: i32, def: i32, power: i32, level: i32) -> i32 {
    //magic numbers from offical formula
    let top_left_bracket = (2 * level) / 5 + 2;
    let atk_over_def: f64 = atk as f64 / def as f64;
    let numerator: f64 = top_left_bracket as f64 * power as f64 * atk_over_def;
    let damage_pre_mod = numerator / 50.0 + 2.0;

    let random_modifier: f64 = rand::thread_rng().gen_range(0.85..=1.0);

    let effecivness_type1: f64 = 1.0; //Matchup(type1)
    let effecivness_type2: f64 = 1.0; //Matchup(type2)
    let type_modifier = effecivness_type1 * effecivness_type2;

    let damage = damage_pre_mod.floor() * random_modifier * type_modifier;

    let final_damage = damage.round() as i32;

    if final_damage == 0 && type_modifier > 0.0 {
        return 1;
    }

    return final_damage;
}

fn use_move(pk_attacker: &mut Pokemon, pk_defender: &mut Pokemon, _move: i32) {
    let atk = pk_attacker.get_atk();
    let def = pk_defender.get_def();
    let level = pk_attacker.get_level();
    let power = _move;

    let damage = calc_damage(atk, def, power, level);
    pk_defender.take_damage(damage);
}

fn main() {
    let pika_sp = Species::new(
        "Pikachu".to_string(),
        Type::Electric,
        Type::None,
        35,
        55,
        40,
        50,
        50,
        90,
        "Lightning Rod".to_string(),
        "Static".to_string(),
        "Hidden".to_string(),
        false,
        50,
        false,
        true,
        10.5,
    );
    let mut pika = Pokemon::new_easy(pika_sp, 50);

    println!("{}", pika.get_hp());
    use_move(&mut pika);
    println!("{}", pika.get_hp());
}
