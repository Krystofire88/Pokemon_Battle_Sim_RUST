use crate::active_pkmn::ActivePokemon;
use crate::field::*;
use crate::helper::*;
use crate::pokemon::Pokemon;
use rand::Rng;

pub struct Battler {
    pokemon_1: Pokemon,
    pokemon_2: Pokemon,
    active_pokemon_1: ActivePokemon,
    active_pokemon_2: ActivePokemon,
    field: Field,
}
impl Battler {
    pub fn new(pokemon_1: Pokemon, pokemon_2: Pokemon) -> Self {
        Self {
            pokemon_1,
            pokemon_2,
            active_pokemon_1: ActivePokemon::new(),
            active_pokemon_2: ActivePokemon::new(),
            field: Field::new(),
        }
    }
    pub fn start(&mut self) {
        self.battle();
    }
    pub fn get_info(&self, pkmn: i32) {
        if pkmn == 1 {
            self.pokemon_1.get_info();
        } else {
            self.pokemon_2.get_info();
        }
    }
    fn battle(&mut self) {
        self.pokemon_1.take_damage(self.damage_calc(
            self.pokemon_2.get_atk(self.active_pokemon_2.get_atk()),
            self.pokemon_1.get_def(self.active_pokemon_1.get_def()),
            self.pokemon_2.get_level(),
            self.pokemon_2.move_set[0].get_power(),
        ));
    }
    fn damage_calc(&self, atk: i32, def: i32, level: i32, power: i32) -> i32 {
        //magic numbers from offical formula
        let top_left_bracket = (2 * level) / 5 + 2;
        let atk_over_def: f64 = atk as f64 / def as f64;
        let numerator: f64 = top_left_bracket as f64 * power as f64 * atk_over_def;
        let damage_pre_mod = numerator / 50.0 + 2.0;

        let random_modifier: f64 = rand::thread_rng().gen_range(0.85..=1.0);

        let effecivness_type1: f64 = 1.0; //Matchup(type1)
        let effecivness_type2: f64 = 1.0; //Matchup(type2)
        let type_modifier = effecivness_type1 * effecivness_type2;

        let damage = damage_pre_mod.floor() * random_modifier * type_modifier;

        let final_damage = damage.round() as i32;

        if final_damage == 0 && type_modifier > 0.0 {
            return 1;
        }

        return final_damage;
    }
    fn check_acc(flip: i32, acc_stage: i32, eva_stage: i32) -> bool {
        if flip == 101 {
            return true;
        }
        let modifier = acc_stage - eva_stage;
        let accuracy = flip as f64 * get_mod_acc(modifier);
        let r = rand::thread_rng().gen_range(1.0..=100.0);

        if r <= accuracy {
            return true;
        }

        return false;
    }
}

//struct TrainerBattler {}
