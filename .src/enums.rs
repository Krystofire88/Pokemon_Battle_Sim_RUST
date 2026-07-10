#[derive(Copy, Clone)]
pub enum Type {
    None,
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}
#[derive(Copy, Clone)]
pub enum Status {
    None,
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic,
    Sleep,
}
#[derive(Copy, Clone)]
pub enum StatusVol {
    None,
    Confusion,
    Infatuation,
    Flinch,
    Torment,
    Drowsy,
}
#[derive(Copy, Clone)]
pub enum Gender {
    Male,
    Female,
    Genderless,
}
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub enum Terrain {
    None,
    Electric,
    Grassy,
    Misty,
    Psychic,
}
#[derive(Copy, Clone)]
pub enum Split {
    Physical,
    Special,
    Status,
}
