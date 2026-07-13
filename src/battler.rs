use crate::active_pkmn;
use crate::active_pkmn::ActivePokemon;
use crate::enums::*;
use crate::field::*;
use crate::helper::*;
use crate::pokemon;
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
        println!(
            "Battle between {} and {} starts!",
            self.pokemon_1.get_name(),
            self.pokemon_2.get_name()
        );
        println!("=========================\n");
        let mut i = 0;
        while self.pokemon_1.get_hp() > 0 && self.pokemon_2.get_hp() > 0 {
            i += 1;
            let mut speed_1: f64 = 0.0;
            let mut speed_2: f64 = 0.0;
            let mut move_index_1: usize;
            let mut move_index_2: usize;
            let mut paralysis: f64 = 1.0;

            println!("Round {i}");

            // pokemon 1
            move_index_1 = rand::thread_rng().gen_range(0..self.pokemon_1.move_set.len());
            if self.pokemon_1.get_status() == Status::Paralysis {
                paralysis = 0.5
            }
            speed_1 = self.pokemon_1.get_spe(self.active_pokemon_1.get_spe()) as f64 * paralysis;

            paralysis = 1.0;

            // pokemon 2
            move_index_2 = rand::thread_rng().gen_range(0..self.pokemon_2.move_set.len());
            if self.pokemon_2.get_status() == Status::Paralysis {
                paralysis = 0.5
            }
            speed_2 = self.pokemon_2.get_spe(self.active_pokemon_2.get_spe()) as f64 * paralysis;

            let priority_1 = self.pokemon_1.move_set[move_index_1].get_priority();
            let priority_2 = self.pokemon_2.move_set[move_index_2].get_priority();

            let first_moves = if priority_1 > priority_2 {
                true
            } else if priority_1 < priority_2 {
                false
            } else if speed_1 > speed_2 {
                true
            } else if speed_1 < speed_2 {
                false
            } else {
                rand::thread_rng().gen_bool(0.5)
            };

            if first_moves {
                self.use_move(first_moves, move_index_1);
                if self.pokemon_2.get_hp() <= 0 {
                    break;
                }
                self.use_move(!first_moves, move_index_2);
            } else {
                self.use_move(first_moves, move_index_2);
                if self.pokemon_1.get_hp() <= 0 {
                    break;
                }
                self.use_move(!first_moves, move_index_1);
            }
            self.pokemon_1.get_info();
            println!("\n{}\n", self.active_pokemon_1.get_atk());
            self.pokemon_2.get_info();
            self.active_pokemon_1.raise_atk(2);
        }
    }
    fn damage_calc(atk: i32, def: i32, level: i32, power: i32) -> i32 {
        //magic numbers from offical formula
        let top_left_bracket = (2 * level) / 5 + 2;
        let atk_over_def: f64 = atk as f64 / def as f64;
        let numerator: f64 = top_left_bracket as f64 * power as f64 * atk_over_def;
        let damage_pre_mod = numerator / 50.0 + 2.0;

        let random_modifier: f64 = rand::thread_rng().gen_range(0.85..=1.0);

        let effecivness_type1: f64 = matchup(type_atk, type_def);
        let effecivness_type2: f64 = matchup(type_atk, type_def);
        let type_modifier = effecivness_type1 * effecivness_type2;

        let damage = damage_pre_mod.floor() * random_modifier * type_modifier;

        let final_damage = damage.round() as i32;

        if final_damage == 0 && type_modifier > 0.0 {
            return 1;
        }
        return final_damage;
    }
    fn use_move(&mut self, first_moves: bool, move_index: usize) {
        let (pokemon_atk, pokemon_def, active_pokemon_atk, active_pokemon_def) = if first_moves {
            (
                &mut self.pokemon_1,
                &mut self.pokemon_2,
                &mut self.active_pokemon_1,
                &mut self.active_pokemon_2,
            )
        } else {
            (
                &mut self.pokemon_2,
                &mut self.pokemon_1,
                &mut self.active_pokemon_2,
                &mut self.active_pokemon_1,
            )
        };
        let mut atk = 0;
        let mut def = 0;

        if !Self::check_acc(
            pokemon_atk.move_set[move_index].get_accuracy(),
            active_pokemon_atk.get_acc(),
            active_pokemon_def.get_eva(),
        ) {
            return;
        }

        if pokemon_atk.move_set[move_index].get_split() == Split::Physical {
            atk = pokemon_atk.get_atk(active_pokemon_atk.get_atk());
            def = pokemon_def.get_def(active_pokemon_def.get_def());
        } else {
            atk = pokemon_atk.get_spa(active_pokemon_atk.get_spa());
            def = pokemon_def.get_spd(active_pokemon_def.get_spd());
        }

        pokemon_def.take_damage(Self::damage_calc(
            atk,
            def,
            pokemon_atk.get_level(),
            pokemon_atk.move_set[move_index].get_power(),
        ));
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
