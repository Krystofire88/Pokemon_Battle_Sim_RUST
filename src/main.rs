mod active_pkmn;
mod battler;
mod consts;
mod damage;
mod enums;
mod field;
mod helper;
mod item;
mod move_effects;
mod moves;
mod pokemon;
mod species;

use battler::Battler;
use consts::*;
use moves::*;
use pokemon::Pokemon;

fn main() {
    poke_println!("Welcome to pokemon battle sim");
    let mut rng = rand::thread_rng(); // init global rng for later seeding

    let mut pika = Pokemon::new_easy(*ALL_SPECIES.get("Pikachu").unwrap(), 50, &mut rng);
    let quick = Move::new(*ALL_MOVES.get("Quick Attack").unwrap());
    let bolt = Move::new(*ALL_MOVES.get("Thunderbolt").unwrap());
    pika.add_move(quick);
    pika.add_move(bolt);

    let mut char = Pokemon::new_easy(*ALL_SPECIES.get("Charizard").unwrap(), 50, &mut rng);
    let flare = Move::new(*ALL_MOVES.get("Flare Blitz").unwrap());
    let air = Move::new(*ALL_MOVES.get("Air Slash").unwrap());
    char.add_move(flare);
    char.add_move(air);

    let mut battler = Battler::new(pika, char, rng);
    battler.get_info(1);
    battler.get_info(2);
    battler.start();
    battler.get_info(1);
    battler.get_info(2);
}
