use rand::Rng;

#[derive(Copy, Clone)]
enum Type {
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
enum Status {
    None,
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic,
    Sleep,
}
#[derive(Copy, Clone)]
enum StatusVol {
    None,
    Confusion,
    Infatuation,
    Flinch,
    Torment,
    Drowsy,
}
#[derive(Copy, Clone)]
enum Gender {
    Male,
    Female,
    Genderless,
}
#[derive(Copy, Clone)]
enum Nature {
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
enum Weather {
    None,
    Sun,
    Rain,
    Sandstorm,
    Snow,
    HarshSun,
    HeavyRain,
    StrongWinds,
}
enum Terrain {
    None,
    Electric,
    Grassy,
    Misty,
    Psychic,
}
enum Split {
    Physical,
    Special,
    Status,
}

#[derive(Clone)]
struct Species {
    name: String,
    type1: Type,
    type2: Type,
    hp: i32,
    atk: i32,
    def: i32,
    spa: i32,
    spd: i32,
    spe: i32,
    ability1: String,
    ability2: String,
    ability_h: String,
    genderless: bool,
    m_to_f_ratio: i32,
    mega: bool,
    gmax: bool,
    weight: f32,
}
impl Species {
    pub fn new(
        name: String,
        type1: Type,
        type2: Type,
        hp: i32,
        atk: i32,
        def: i32,
        spa: i32,
        spd: i32,
        spe: i32,
        ability1: String,
        ability2: String,
        ability_h: String,
        genderless: bool,
        m_to_f_ratio: i32,
        mega: bool,
        gmax: bool,
        weight: f32,
    ) -> Species {
        Species {
            name: name,
            type1: type1,
            type2: type2,
            hp: hp,
            atk: atk,
            def: def,
            spa: spa,
            spd: spd,
            spe: spe,
            ability1: ability1,
            ability2: ability2,
            ability_h: ability_h,
            genderless: genderless,
            m_to_f_ratio: m_to_f_ratio,
            mega: mega,
            gmax: gmax,
            weight: weight,
        }
    }
}

#[derive(Clone)]
struct Pokemon {
    species: Species,
    nickname: String,
    gender: Gender,
    level: i32,
    max_hp: i32,
    hp: i32,
    ability: i32,
    non_volitile_status: Status,
    volitile_status: Vec<StatusVol>,
    hp_iv: i32,
    hp_ev: i32,
    atk_iv: i32,
    atk_ev: i32,
    def_iv: i32,
    def_ev: i32,
    spa_iv: i32,
    spa_ev: i32,
    spd_iv: i32,
    spd_ev: i32,
    spe_iv: i32,
    spe_ev: i32,
    atk_mod: i32,
    def_mod: i32,
    spa_mod: i32,
    spd_mod: i32,
    spe_mod: i32,
    acc_mod: i32,
    eva_mod: i32,
    nature: Nature,
    held_item: Item,
    gmax: bool,
    is_dmax: bool,
    tera_type: Type,
    terasallized: bool,
}

impl Pokemon {
    fn new(
        species: Species,
        nickname: String,
        gender: Gender,
        level: i32,
        ability: i32,
        hp_iv: i32,
        hp_ev: i32,
        atk_iv: i32,
        atk_ev: i32,
        def_iv: i32,
        def_ev: i32,
        spa_iv: i32,
        spa_ev: i32,
        spd_iv: i32,
        spd_ev: i32,
        spe_iv: i32,
        spe_ev: i32,
        nature: Nature,
        held_item: Item,
        gmax: bool,
        tera_type: Type,
    ) -> Self {
        let mut pk = Self {
            species,
            nickname,
            gender,
            level,
            max_hp: 0,
            hp: 0,
            ability,
            non_volitile_status: Status::None,
            volitile_status: Vec::new(),
            hp_iv,
            hp_ev,
            atk_iv,
            atk_ev,
            def_iv,
            def_ev,
            spa_iv,
            spa_ev,
            spd_iv,
            spd_ev,
            spe_iv,
            spe_ev,
            atk_mod: 0,
            def_mod: 0,
            spa_mod: 0,
            spd_mod: 0,
            spe_mod: 0,
            acc_mod: 0,
            eva_mod: 0,
            nature,
            held_item,
            gmax,
            is_dmax: false,
            tera_type,
            terasallized: false,
        };

        pk.max_hp = pk.calc_hp();
        pk.hp = pk.max_hp;

        return pk;
    }
    fn new_easy(species: Species, level: i32) -> Self {
        let mut pk = Self {
            species,
            nickname: "Placeholder".to_string(),
            gender: Gender::Genderless,
            level,
            max_hp: 0,
            hp: 0,
            ability: rand::thread_rng().gen_range(0..=3),
            non_volitile_status: Status::None,
            volitile_status: Vec::new(),
            hp_iv: rand::thread_rng().gen_range(0..=31),
            hp_ev: rand::thread_rng().gen_range(0..=31),
            atk_iv: rand::thread_rng().gen_range(0..=31),
            atk_ev: rand::thread_rng().gen_range(0..=31),
            def_iv: rand::thread_rng().gen_range(0..=31),
            def_ev: rand::thread_rng().gen_range(0..=31),
            spa_iv: rand::thread_rng().gen_range(0..=31),
            spa_ev: rand::thread_rng().gen_range(0..=31),
            spd_iv: rand::thread_rng().gen_range(0..=31),
            spd_ev: rand::thread_rng().gen_range(0..=31),
            spe_iv: rand::thread_rng().gen_range(0..=31),
            spe_ev: rand::thread_rng().gen_range(0..=31),
            atk_mod: 0,
            def_mod: 0,
            spa_mod: 0,
            spd_mod: 0,
            spe_mod: 0,
            acc_mod: 0,
            eva_mod: 0,
            nature: Nature::Serious,
            held_item: Item {},
            gmax: false,
            is_dmax: false,
            tera_type: Type::Normal,
            terasallized: false,
        };

        pk.max_hp = pk.calc_hp();
        pk.hp = pk.max_hp;

        if !pk.species.genderless {
            let flip = rand::thread_rng().gen_range(0..100);
            if flip > pk.species.m_to_f_ratio {
                pk.gender = Gender::Female;
            } else {
                pk.gender = Gender::Male;
            }
        }

        return pk;
    }
    fn calc_hp(&self) -> i32 {
        //magic numbers from offical formula
        let mut hp_evs: f64 = self.hp_ev as f64 / 4.0;
        hp_evs = hp_evs.floor();
        let numerator_left: f64 = 2.0 * self.species.hp as f64 + self.hp_iv as f64 + hp_evs;
        let numerator: f64 = numerator_left * self.level as f64;
        let floor: f64 = numerator / 100.0;
        return floor.floor() as i32 + self.level + 10;
    }
}

#[derive(Copy, Clone)]
struct Item {}

fn calc_damage(atk: i32, def: i32, power: i32, level: i32) -> i32 {
    //magic numbers from offical formula
    let top_left_bracket = (2 * level) / 5 + 2;
    let atk_over_def: f64 = atk as f64 / def as f64;
    let numerator: f64 = top_left_bracket as f64 * power as f64 * atk_over_def;
    let damage_pre_mod = numerator / 50.0 + 2.0;

    let damage = damage_pre_mod.round() as i32 * 1;
    return damage;
}

fn inflict_damage(mon: &mut Pokemon) {
    let damage = calc_damage(100, 80, 80, 50);
    mon.hp -= damage;
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
