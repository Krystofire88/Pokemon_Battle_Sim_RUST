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
    m_to_f_ratio: f32,
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
        m_to_f_ratio: f32,
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
    gender: bool,
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
    atk: i32,
    def: i32,
    spa: i32,
    spd: i32,
    spe: i32,
    acc: i32,
    eva: i32,
    nature: String,
    held_item: Item,
    gmax: bool,
    is_dmax: bool,
    tera_type: Type,
    terasallized: bool,
}

impl Pokemon {
    fn new(species: Species, nickname: String, gender: bool, level: i32, max_hp: i32, hp: i32, ability: i32, non_volitile_status: Status, volitile_status: Vec<StatusVol>, hp_iv: i32, hp_ev: i32, atk_iv: i32, atk_ev: i32, def_iv: i32, def_ev: i32, spa_iv: i32, spa_ev: i32, spd_iv: i32, spd_ev: i32, spe_iv: i32, spe_ev: i32, atk: i32, def: i32, spa: i32, spd: i32, spe: i32, acc: i32, eva: i32, nature: String, held_item: Item, gmax: bool, is_dmax: bool, tera_type: Type, terasallized: bool) -> Self {
        Self { species, nickname, gender, level, max_hp, hp, ability, non_volitile_status, volitile_status, hp_iv, hp_ev, atk_iv, atk_ev, def_iv, def_ev, spa_iv, spa_ev, spd_iv, spd_ev, spe_iv, spe_ev, atk, def, spa, spd, spe, acc, eva, nature, held_item, gmax, is_dmax, tera_type, terasallized }
    }
}

#[derive(Copy, Clone)]
struct Item {}

fn calc_damage(atk: i32, def: i32, power: i32, level: i32) -> i32 {
    let top_left_bracket = (2 * level) / 5 + 2; //magic numbers from offical formula
    let atk_over_def: f64 = atk as f64 / def as f64;
    let numerator: f64 = top_left_bracket as f64 * power as f64 * atk_over_def;
    let damage_pre_mod = numerator / 50.0 + 2.0; //magic numbers from offical formula

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
        0.5,
        false,
        true,
        10.5,
    );
    let mut pika = Pokemon::new(pika_sp, "Mouse".to_string(), true, 50, max_hp, hp, ability, non_volitile_status, volitile_status, hp_iv, hp_ev, atk_iv, atk_ev, def_iv, def_ev, spa_iv, spa_ev, spd_iv, spd_ev, spe_iv, spe_ev, atk, def, spa, spd, spe, acc, eva, nature, held_item, gmax, is_dmax, tera_type, terastallized)

    inflict_damage(&mut pika);
}
