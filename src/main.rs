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

use battler::Battler;
use consts::*;
use moves::*;
use pokemon::Pokemon;
fn main() {
    poke_println!("Welcome to pokemon battle sim");

    let mut pika = Pokemon::new_easy(*ALL_SPECIES.get("Pikachu").unwrap(), 50);
    let tck = Move::new(0);
    let swr = Move::new(1);
    pika.add_move(tck);
    pika.add_move(swr);

    let mut char = Pokemon::new_easy(*ALL_SPECIES.get("Charizard").unwrap(), 50);
    let tak = Move::new(0);
    let swd = Move::new(1);
    char.add_move(tak);
    char.add_move(swd);

    poke_println!("AHhhhhhhhh");
    let mut battler = Battler::new(pika, char);
    battler.get_info(1);
    battler.get_info(2);
    battler.start();
    battler.get_info(1);
    battler.get_info(2);
    poke_println!("AHhhhhhhhh");
}
