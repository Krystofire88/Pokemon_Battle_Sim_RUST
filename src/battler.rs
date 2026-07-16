use crate::active_pkmn::ActivePokemon;
use crate::consts::*;
use crate::damage::*;
use crate::enums::*;
use crate::field::*;
use crate::helper::*;
use crate::move_effects::*;
use crate::poke_println;
use crate::pokemon::Pokemon;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Battler {
    pokemon_1: Pokemon,
    pokemon_2: Pokemon,
    active_pokemon_1: ActivePokemon,
    active_pokemon_2: ActivePokemon,
    field: Field,
    rng: ThreadRng,
}
impl Battler {
    pub fn new(pokemon_1: Pokemon, pokemon_2: Pokemon, rng: ThreadRng) -> Self {
        Self {
            pokemon_1,
            pokemon_2,
            active_pokemon_1: ActivePokemon::new(),
            active_pokemon_2: ActivePokemon::new(),
            field: Field::new(),
            rng,
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
        poke_println!(
            "Battle between {} and {} starts!",
            self.pokemon_1.get_nickname(),
            self.pokemon_2.get_nickname()
        );
        poke_println!("=========================\n");
        let mut i = 0;
        while self.pokemon_1.get_hp() > 0 && self.pokemon_2.get_hp() > 0 {
            i += 1;

            let mut paralysis: f64 = 1.0;
            let mut tailwind: f64 = 1.0;
            let mut trick_room: f64 = 1.0;

            poke_println!("\nRound {i}");

            if self.field.is_trick_room() {
                trick_room = -1.0;
            }

            // pokemon 1
            let move_index_1 = self.rng.gen_range(0..self.pokemon_1.move_set.len());
            if self.pokemon_1.get_status() == Status::Paralysis {
                paralysis = 0.5;
            }
            if self.field.field_side_a.is_tailwind() {
                tailwind = 2.0;
            }

            let speed_1 = self.pokemon_1.get_spe() as f64
                * get_mod(self.active_pokemon_1.get_spe())
                * paralysis
                * tailwind
                * trick_room;

            paralysis = 1.0;
            tailwind = 1.0;

            // pokemon 2
            let move_index_2 = self.rng.gen_range(0..self.pokemon_2.move_set.len());
            if self.pokemon_2.get_status() == Status::Paralysis {
                paralysis = 0.5
            }
            if self.field.field_side_b.is_tailwind() {
                tailwind = 2.0;
            }
            let speed_2 = self.pokemon_2.get_spe() as f64
                * get_mod(self.active_pokemon_2.get_spe())
                * paralysis
                * tailwind
                * trick_room;

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
                self.rng.gen_bool(0.5)
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

            Self::post_turn_check(&mut self.pokemon_1, &mut self.active_pokemon_1, &self.field);
            Self::post_turn_check(&mut self.pokemon_2, &mut self.active_pokemon_2, &self.field);
            self.field.step_timers();
        }
    }
    fn post_turn_check(pokemon: &mut Pokemon, active_pokemon: &mut ActivePokemon, field: &Field) {
        match pokemon.get_status() {
            Status::Burn => pokemon.take_chip_damage(16),
            Status::Poison => pokemon.take_chip_damage(8),
            Status::Toxic => pokemon.take_toxic_damage(active_pokemon.get_toxic_timer()),
            _ => (),
        }
        for status in active_pokemon.get_statuses() {
            match status {
                StatusVol::Drowsy { asleep_next } => {
                    if *asleep_next {
                        pokemon.inflict_status(Status::Sleep)
                    }
                }
                _ => (),
            }
        }
        if pokemon.get_status() == Status::Sleep {
            active_pokemon.remove_status(StatusVol::Drowsy { asleep_next: true });
        }

        active_pokemon.drop_protect();
        active_pokemon.step_timers();
        match field.get_terrain() {
            Terrain::Grassy => pokemon.heal(16),
            _ => (),
        }
    }
    fn use_move(&mut self, is_attacker_1: bool, move_index: usize) {
        let (pokemon_atk, pokemon_def, active_pokemon_atk, active_pokemon_def) = if is_attacker_1 {
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

        if pokemon_atk.move_set[move_index].get_pp() <= 0 {
            if !active_pokemon_def.is_protected() {
                let damage = damage(
                    (pokemon_atk.get_atk() as f64 * get_mod(active_pokemon_atk.get_atk())) as i32,
                    (pokemon_def.get_def() as f64 * get_mod(active_pokemon_def.get_def())) as i32,
                    pokemon_atk.get_level(),
                    50,
                    DamageModifiers::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0),
                    pokemon_def.get_hp(),
                );

                poke_println!(
                    "{} used struggle against {} it did {} damage!",
                    pokemon_atk.get_nickname(),
                    pokemon_def.get_nickname(),
                    damage
                );

                pokemon_def.take_damage(damage);
                pokemon_atk.take_chip_damage(4);
                return;
            } else {
                poke_println!(
                    "{} used {} against {} but {} was protected!",
                    pokemon_atk.get_nickname(),
                    pokemon_atk.move_set[move_index].get_name(),
                    pokemon_def.get_nickname(),
                    pokemon_def.get_nickname()
                );
                return;
            }
        }

        pokemon_atk.move_set[move_index].lose_pp(1);

        if !Self::check_acc(
            &mut self.rng,
            pokemon_atk.move_set[move_index].get_accuracy(),
            active_pokemon_atk.get_acc(),
            active_pokemon_def.get_eva(),
        ) {
            return;
        }

        if pokemon_atk.move_set[move_index].get_split() == Split::Status {
            poke_println!(
                "{} used {}",
                pokemon_atk.get_nickname(),
                pokemon_atk.move_set[move_index].get_name()
            );
            Self::use_status_moves(
                active_pokemon_atk,
                active_pokemon_def,
                pokemon_atk,
                pokemon_def,
                &mut self.rng,
                move_index,
            );
            return;
        }

        let hits = pokemon_atk.move_set[move_index].get_hits(&mut self.rng);

        for _ in 0..hits {
            if !active_pokemon_def.is_protected() {
                let damage = damage_calc(
                    &mut self.rng,
                    pokemon_atk,
                    pokemon_def,
                    active_pokemon_atk,
                    active_pokemon_def,
                    &pokemon_atk.move_set[move_index],
                    self.field.get_weather(),
                    self.field.get_terrain(),
                );

                Self::use_status_moves(
                    active_pokemon_atk,
                    active_pokemon_def,
                    pokemon_atk,
                    pokemon_def,
                    &mut self.rng,
                    move_index,
                );

                poke_println!(
                    "{} used {} against {} it did {} damage!",
                    pokemon_atk.get_nickname(),
                    pokemon_atk.move_set[move_index].get_name(),
                    pokemon_def.get_nickname(),
                    damage
                );

                pokemon_def.take_damage(damage);
                if pokemon_def.get_hp() <= 0 {
                    break;
                }
            } else {
                poke_println!(
                    "{} used {} against {} but {} was protected!",
                    pokemon_atk.get_nickname(),
                    pokemon_atk.move_set[move_index].get_name(),
                    pokemon_def.get_nickname(),
                    pokemon_def.get_nickname()
                );
            }
        }
    }
    fn use_status_moves(
        active_pokemon_atk: &mut ActivePokemon,
        active_pokemon_def: &mut ActivePokemon,
        pokemon_atk: &mut Pokemon,
        pokemon_def: &mut Pokemon,
        rng: &mut ThreadRng,
        move_index: usize,
    ) {
        for effect in pokemon_atk.move_set[move_index].get_effects().clone() {
            if effect.get_effect_chance() != CANNOT_MISS {
                let r = rng.gen_range(1..=100);
                if r > effect.get_effect_chance() {
                    continue;
                }
            }
            if effect.get_target() == Target::User {
                Self::use_status(effect.get_effect(), active_pokemon_atk, pokemon_atk, rng);
            } else if effect.get_target() == Target::Opponent {
                if !active_pokemon_def.is_protected() {
                    Self::use_status(effect.get_effect(), active_pokemon_def, pokemon_def, rng);
                }
            } else if effect.get_target() == Target::All {
                Self::use_status(effect.get_effect(), active_pokemon_atk, pokemon_atk, rng);
                if !active_pokemon_def.is_protected() {
                    Self::use_status(effect.get_effect(), active_pokemon_def, pokemon_def, rng);
                }
            }
        }
    }
    fn use_status(
        effect: Effect,
        active_pokemon: &mut ActivePokemon,
        pokemon: &mut Pokemon,
        rng: &mut ThreadRng,
    ) {
        match effect {
            Effect::ChangeStat { stat, stages } => active_pokemon.change_stat(stat, stages),
            Effect::InflictStatus { status } => pokemon.inflict_status(status),
            Effect::InflictStatusVol { status } => active_pokemon.inflict_status(status),
            Effect::Heal { fraction } => pokemon.heal(fraction),
            Effect::Protect => active_pokemon.protect(rng),
            _ => (),
        };
    }
    fn check_acc(rng: &mut ThreadRng, flip: i32, acc_stage: i32, eva_stage: i32) -> bool {
        if flip == CANNOT_MISS {
            return true;
        }
        let mut modifier = acc_stage - eva_stage;
        modifier = modifier.clamp(-6, 6);
        let accuracy = flip as f64 * get_mod_acc(modifier);
        let r = rng.gen_range(1.0..=100.0);

        if r <= accuracy {
            return true;
        }
        false
    }
}

//struct TrainerBattler {}
