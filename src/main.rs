mod active_pkmn;
mod battler;
mod consts;
mod enums;
mod field;
mod helper;
mod item;
mod move_effects;
mod moves;
mod pokemon;
mod species;

use consts::*;
use enums::*;
use pokemon::Pokemon;
use serde::Deserialize;
use species::Species;
use std::fs;

use crate::{
    battler::Battler,
    moves::{Move, MoveBase},
};

fn main() {
    println!("Welcome to pokemon battle sim");

    let mut pika = Pokemon::new_easy(ALL_SPECIES.get("Pikachu").unwrap(), 50);
    let tck = Move::new(0);
    pika.add_move(tck);

    let mut char = Pokemon::new_easy(ALL_SPECIES.get("Charizard").unwrap(), 50);
    let tak = Move::new(0);
    char.add_move(tak);

    let mut battler = Battler::new(pika, char);
    battler.get_info(1);
    battler.start();
    battler.get_info(1);
}
