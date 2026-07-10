mod enums;
mod helper;
mod item;
mod pokemon;
mod species;

use enums::*;
use pokemon::Pokemon;
use serde::Deserialize;
use species::Species;
use std::fs;

fn main() {
    println!("Welcome to pokemon battle sim");

    let data =
        fs::read_to_string("src/Pokemon_Jsons/AllPokemon.json").expect("Could not read file");

    let all_species: Vec<Species> = serde_json::from_str(&data).expect("Invalid JSON");

    println!("{}", all_species.len());
}
