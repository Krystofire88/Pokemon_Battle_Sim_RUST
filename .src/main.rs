mod enums;
mod field;
mod helper;
mod item;
mod move_effects;
mod moves;
mod pokemon;
mod species;

use enums::*;
use helper::*;
use item::Item;
use move_effects::MoveEffects;
use moves::MoveBase;
use pokemon::Pokemon;
use rand::Rng;
use species::Species;

use crate::moves::Move;

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

fn use_move(pk_attacker: &mut Pokemon, pk_defender: &mut Pokemon, _move: &mut Move) {
    if _move.get_pp() < 1 {
        let struggle = MoveBase::new(
            "Struggle".to_string(),
            Type::Normal,
            50,
            Split::Physical,
            101,
            1,
            0,
            true,
            false,
            Vec::new(),
        );
    }
    if check_acc(
        _move.move_base.get_accuracy(),
        pk_attacker.get_acc(),
        pk_defender.get_eva(),
    ) {
        _move.lose_pp(1);
        let atk = pk_attacker.get_atk();
        let def = pk_defender.get_def();
        let level = pk_attacker.get_level();
        let power = _move.move_base.get_power();

        let damage = calc_damage(atk, def, power, level);
        println!("{}", damage);
        pk_defender.take_damage(damage);
    }
}

fn check_acc(flip: i32, acc_stage: i32, eva_stage: i32) -> bool {
    if flip == 101 {
        return true;
    }
    let modifier = acc_stage - eva_stage;
    let accuracy = flip as f64 * get_mod_acc(modifier);
    let r = rand::thread_rng().gen_range(1.0..=100.0);

    if r <= accuracy {
        return true;
    }

    return false;
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

    let charizard_sp = Species::new(
        "Charizard".to_string(),
        Type::Electric,
        Type::Flying,
        78,
        84,
        78,
        109,
        85,
        100,
        "Blaze".to_string(),
        "".to_string(),
        "Solar Power".to_string(),
        false,
        87,
        true,
        true,
        90.5,
    );
    let mut char = Pokemon::new_easy(charizard_sp, 50);

    let mvb = MoveBase::new(
        "Tackle".to_string(),
        Type::Normal,
        40,
        Split::Physical,
        100,
        40,
        0,
        true,
        false,
        Vec::new(),
    );
    let mut tackle = Move::new(mvb);

    println!("{}", pika.get_hp());
    println!("{}", tackle.get_pp());
    use_move(&mut char, &mut pika, &mut tackle);
    println!("{}", tackle.move_base.get_name());
    println!("{}", tackle.get_pp());
    println!("{}", pika.get_hp());
}
