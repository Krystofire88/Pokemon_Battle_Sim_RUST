use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, PartialEq, Eq, Debug)]
pub enum Type {
    None = 0,
    Normal = 1,
    Fire = 2,
    Water = 3,
    Electric = 4,
    Grass = 5,
    Ice = 6,
    Fighting = 7,
    Poison = 8,
    Ground = 9,
    Flying = 10,
    Psychic = 11,
    Bug = 12,
    Rock = 13,
    Ghost = 14,
    Dragon = 15,
    Dark = 16,
    Steel = 17,
    Fairy = 18,
}
#[derive(Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum Status {
    None,
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic,
    Sleep,
}
#[derive(Copy, Clone, PartialEq, Eq, Deserialize, Hash)]
pub enum StatusVol {
    None,
    Confusion,
    Infatuation,
    Flinch,
    Torment,
    Drowsy { asleep_next: bool },
}
#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Male,
    Female,
    Genderless,
}
#[derive(Copy, Clone, Debug)]
pub enum Nature {
    // + attack
    Hardy,   // neutral
    Lonely,  // - defense
    Adamant, // - sp attack
    Naughty, // - sp defense
    Brave,   // - speed

    // + defense
    Bold,    // - attack
    Docile,  // neutral
    Impish,  // - sp attack
    Lax,     // - sp defense
    Relaxed, // - speed

    // + sp attack
    Modest,  // - attack
    Mild,    // - defense
    Bashful, // neutral
    Rash,    // - sp defense
    Quiet,   // - speed

    // + sp defense
    Calm,    // - attack
    Gentle,  // - defense
    Careful, // - sp attack
    Quirky,  // neutral
    Sassy,   // - speed

    // + speed
    Timid,   // - attack
    Hasty,   // - defense
    Jolly,   // - sp attack
    Naive,   // - sp defense
    Serious, // neutral
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Weather {
    None,
    Sun,
    Rain,
    Sandstorm,
    Snow,
    HarshSun,
    HeavyRain,
    StrongWinds,
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Terrain {
    None,
    Electric,
    Grassy,
    Misty,
    Psychic,
}
#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum Split {
    Physical,
    Special,
    Status,
}
#[derive(Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum Stat {
    Atk,
    Def,
    Spa,
    Spd,
    Spe,
    Acc,
    Eva,
}
