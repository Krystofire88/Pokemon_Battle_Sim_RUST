use crate::enums::StatusVol;

pub struct ActivePokemon {
    volitile_status: Vec<StatusVol>,
    atk_mod: i32,
    def_mod: i32,
    spa_mod: i32,
    spd_mod: i32,
    spe_mod: i32,
    acc_mod: i32,
    eva_mod: i32,
    is_dmax: bool,
}
impl ActivePokemon {
    pub fn new() -> Self {
        Self {
            volitile_status: Vec::new(),
            atk_mod: 0,
            def_mod: 0,
            spa_mod: 0,
            spd_mod: 0,
            spe_mod: 0,
            acc_mod: 0,
            eva_mod: 0,
            is_dmax: false,
        }
    }
    pub fn get_atk(&self) -> i32 {
        self.atk_mod
    }
    pub fn change_atk(&mut self, stage: i32) {
        self.atk_mod += stage;
        self.atk_mod = self.atk_mod.clamp(-6, 6);
    }
    pub fn get_def(&self) -> i32 {
        self.def_mod
    }
    pub fn change_def(&mut self, stage: i32) {
        self.def_mod += stage;
        self.def_mod = self.def_mod.clamp(-6, 6);
    }
    pub fn get_spa(&self) -> i32 {
        self.spa_mod
    }
    pub fn change_spa(&mut self, stage: i32) {
        self.spa_mod += stage;
        self.spa_mod = self.spa_mod.clamp(-6, 6);
    }
    pub fn get_spd(&self) -> i32 {
        self.spd_mod
    }
    pub fn change_spd(&mut self, stage: i32) {
        self.spd_mod += stage;
        self.spd_mod = self.spd_mod.clamp(-6, 6);
    }
    pub fn get_spe(&self) -> i32 {
        self.spe_mod
    }
    pub fn change_spe(&mut self, stage: i32) {
        self.spe_mod += stage;
        self.spe_mod = self.spe_mod.clamp(-6, 6);
    }
    pub fn get_acc(&self) -> i32 {
        self.acc_mod
    }
    pub fn change_acc(&mut self, stage: i32) {
        self.acc_mod += stage;
        self.acc_mod = self.acc_mod.clamp(-6, 6);
    }
    pub fn get_eva(&self) -> i32 {
        self.eva_mod
    }
    pub fn change_eva(&mut self, stage: i32) {
        self.eva_mod += stage;
        self.eva_mod = self.eva_mod.clamp(-6, 6);
    }
    pub fn get_dmax(&self) -> bool {
        self.is_dmax
    }
    pub fn inflict_status(&mut self, status: StatusVol) {
        self.volitile_status.push(status);
    }
}
