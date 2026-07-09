mod enums;
mod field;
mod item;
mod moves;
mod pokemon;
mod species;

use enums::*;
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

    let type_modifier = 1.0;

    let damage = damage_pre_mod.floor() * random_modifier * type_modifier;

    let final_damage = damage.round() as i32;

    if final_damage == 0 && type_modifier > 0.0 {
        return 1;
    }

    return final_damage;
}

fn inflict_damage(mon: &mut Pokemon) {
    let damage = calc_damage(100, 80, 80, 50);
    mon.hp -= damage;
    if mon.hp < 0 {
        mon.hp = 0
    }
}

fn main() {
    let pika_sp = Species::new(
        "Pikachu".to_string(),
        Type::Electric,
        Type::None,
        60,
        60,
        80,
        100,
        80,
        80,
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

    inflict_damage(&mut pika);
}
