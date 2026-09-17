use std::io::SeekFrom;

use crate::enums::*;
use crate::field::Field;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct ActivePokemon {
    volatile_status: Vec<StatusVol>,
    atk_mod: i8,
    def_mod: i8,
    spa_mod: i8,
    spd_mod: i8,
    spe_mod: i8,
    acc_mod: i8,
    eva_mod: i8,
    is_dmax: bool,
    crit_stage: u8,
    is_protected: bool,
    protect_times: u32,
    toxic_timer: u32,
}
impl ActivePokemon {
    pub fn new() -> Self {
        Self {
            volatile_status: Vec::new(),
            atk_mod: 0,
            def_mod: 0,
            spa_mod: 0,
            spd_mod: 0,
            spe_mod: 0,
            acc_mod: 0,
            eva_mod: 0,
            is_dmax: false,
            crit_stage: 0,
            is_protected: false,
            protect_times: 0,
            toxic_timer: 0,
        }
    }
    pub fn change_stat(&mut self, stat: Stat, stage: i8) {
        match stat {
            Stat::Atk => self.change_atk(stage),
            Stat::Def => self.change_def(stage),
            Stat::Spa => self.change_spa(stage),
            Stat::Spd => self.change_spd(stage),
            Stat::Spe => self.change_spe(stage),
            Stat::Acc => self.change_acc(stage),
            Stat::Eva => self.change_eva(stage),
        }
    }
    pub fn get_atk(&self) -> i8 {
        self.atk_mod
    }
    fn change_atk(&mut self, stage: i8) {
        self.atk_mod += stage;
        self.atk_mod = self.atk_mod.clamp(-6, 6);
    }
    pub fn get_def(&self) -> i8 {
        self.def_mod
    }
    fn change_def(&mut self, stage: i8) {
        self.def_mod += stage;
        self.def_mod = self.def_mod.clamp(-6, 6);
    }
    pub fn get_spa(&self) -> i8 {
        self.spa_mod
    }
    fn change_spa(&mut self, stage: i8) {
        self.spa_mod += stage;
        self.spa_mod = self.spa_mod.clamp(-6, 6);
    }
    pub fn get_spd(&self) -> i8 {
        self.spd_mod
    }
    fn change_spd(&mut self, stage: i8) {
        self.spd_mod += stage;
        self.spd_mod = self.spd_mod.clamp(-6, 6);
    }
    pub fn get_spe(&self) -> i8 {
        self.spe_mod
    }
    fn change_spe(&mut self, stage: i8) {
        self.spe_mod += stage;
        self.spe_mod = self.spe_mod.clamp(-6, 6);
    }
    pub fn get_acc(&self) -> i8 {
        self.acc_mod
    }
    fn change_acc(&mut self, stage: i8) {
        self.acc_mod += stage;
        self.acc_mod = self.acc_mod.clamp(-6, 6);
    }
    pub fn get_eva(&self) -> i8 {
        self.eva_mod
    }
    fn change_eva(&mut self, stage: i8) {
        self.eva_mod += stage;
        self.eva_mod = self.eva_mod.clamp(-6, 6);
    }
    pub fn get_dmax(&self) -> bool {
        self.is_dmax
    }
    pub fn inflict_status(&mut self, status_vol: StatusVol, field: &Field, status: Status) {
        match (status_vol, field.get_terrain(), status) {
            (StatusVol::Confusion { .. }, Terrain::Misty, _) => return,
            (StatusVol::Drowsy { .. }, Terrain::Electric, _) => return,
            (StatusVol::Drowsy { .. }, _, Status::Sleep) => return,
            _ => (),
        }

        let already_has = self
            .volatile_status
            .iter()
            .any(|s| std::mem::discriminant(s) == std::mem::discriminant(&status_vol));

        if !already_has {
            self.volatile_status.push(status_vol);
        }
    }
    pub fn remove_status(&mut self, status: StatusVol) {
        self.volatile_status.retain(|s| *s != status)
    }
    pub fn get_statuses(&self) -> &Vec<StatusVol> {
        &self.volatile_status
    }
    pub fn get_status(&self, status: StatusVol) -> bool {
        self.volatile_status.contains(&status)
    }
    pub fn get_crit_stage(&self) -> u8 {
        self.crit_stage
    }
    pub fn reset_crit_stage(&mut self) {
        self.crit_stage = 0
    }
    pub fn raise_crit_stage(&mut self, stage: u8) {
        let mut stg = stage;
        stg = stg.clamp(0, 2); // +3 is only achivable in gen 5 with the wonder launcher
        self.crit_stage += stg;
        self.crit_stage = self.crit_stage.clamp(0, 3); // 4+ is achievable but has no effect above gen VI
    }
    pub fn protect(&mut self, rng: &mut ThreadRng) {
        if self.protect_times > 0 {
            let three: u32 = 3;
            let chance = three.saturating_pow(self.protect_times);
            if rng.gen_range(1..=chance) == 1 {
                self.is_protected = true;
                self.protect_times += 1;
            } else {
                self.drop_protect();
            }
        } else {
            self.is_protected = true;
            self.protect_times += 1;
        }
    }
    pub fn drop_protect(&mut self) {
        if self.is_protected {
            self.is_protected = false
        } else {
            self.protect_times = 0;
        }
    }
    pub fn is_protected(&self) -> bool {
        self.is_protected
    }
    pub fn get_toxic_timer(&self) -> u32 {
        self.toxic_timer
    }
    pub fn step_timers(&mut self) {
        if self.toxic_timer > 0 {
            self.toxic_timer += 1
        }
        // more timers
    }
    pub fn step_drowsy(&mut self) -> bool {
        let mut fell_asleep = false;
        for s in self.volatile_status.iter_mut() {
            if let StatusVol::Drowsy { asleep_next } = s {
                if *asleep_next {
                    fell_asleep = true;
                } else {
                    *asleep_next = true;
                }
            }
        }
        if fell_asleep {
            self.volatile_status
                .retain(|s| !matches!(s, StatusVol::Drowsy { .. }));
        }
        fell_asleep
    }
}
