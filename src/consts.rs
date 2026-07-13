use crate::moves::MoveBase;
use crate::species::Species;
use std::collections::HashMap;
use std::fs;
use std::sync::LazyLock;

pub static ALL_SPECIES: LazyLock<HashMap<String, Species>> = LazyLock::new(|| {
    let species: Vec<Species> =
        serde_json::from_str(&fs::read_to_string("src/Pokemon_Jsons/AllPokemon.json").unwrap())
            .unwrap();

    species.into_iter().map(|s| (s.get_name(), s)).collect()
});

pub static ALL_MOVES: LazyLock<HashMap<String, MoveBase>> = LazyLock::new(|| {
    let species: Vec<MoveBase> =
        serde_json::from_str(&fs::read_to_string("src/Pokemon_Jsons/AllPokemon.json").unwrap())
            .unwrap();

    species.into_iter().map(|s| (s.get_name(), s)).collect()
});

pub static ALL_SPECIES_VEC: LazyLock<Vec<Species>> = LazyLock::new(|| {
    let data = fs::read_to_string("src/Pokemon_Jsons/AllPokemon.json")
        .expect("Could not read AllPokemon.json");

    serde_json::from_str(&data).expect("Invalid Pokémon JSON")
});

pub static ALL_MOVES_VEC: LazyLock<Vec<MoveBase>> = LazyLock::new(|| {
    let data = fs::read_to_string("src/Pokemon_Jsons/AllMoves.json")
        .expect("Could not read AllMoves.json");

    serde_json::from_str(&data).expect("Invalid Moves JSON")
});
