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
    let mut rng = rand::thread_rng();

    let mut pika = Pokemon::new_easy(*ALL_SPECIES.get("Pikachu").unwrap(), 50, &mut rng);
    let tck = Move::new(0);
    let swr = Move::new(1);
    pika.add_move(tck);
    pika.add_move(swr);

    let mut char = Pokemon::new_easy(*ALL_SPECIES.get("Charizard").unwrap(), 50, &mut rng);
    let tak = Move::new(0);
    let swd = Move::new(1);
    char.add_move(tak);
    char.add_move(swd);

    let mut battler = Battler::new(pika, char, rng);
    battler.get_info(1);
    battler.get_info(2);
    battler.start();
    battler.get_info(1);
    battler.get_info(2);
}
