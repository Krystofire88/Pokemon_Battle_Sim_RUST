use crate::enums::*;

#[derive(Clone)]
pub struct FieldSide {
    stealth_rock: bool,
    sharp_steel: bool,
    sticky_web: bool,
    spikes: i32,
    toxic_spikes: i32,
    reflect_timer: i32,
    light_screen_timer: i32,
    aurora_veil_timer: i32,
    tailwind_timer: i32,
    mist_timer: i32,
    safeguard_timer: i32,
}

impl FieldSide {
    pub fn new() -> Self {
        Self {
            stealth_rock: false,
            sharp_steel: false,
            sticky_web: false,
            spikes: 0,
            toxic_spikes: 0,
            reflect_timer: 0,
            light_screen_timer: 0,
            aurora_veil_timer: 0,
            tailwind_timer: 0,
            mist_timer: 0,
            safeguard_timer: 0,
        }
    }
    pub fn is_tailwind(&self) -> bool {
        self.tailwind_timer > 0
    }
    pub fn step_timer(&mut self) {
        if self.reflect_timer > 0 {
            self.reflect_timer -= 1;
        }
        if self.light_screen_timer > 0 {
            self.light_screen_timer -= 1;
        }
        if self.aurora_veil_timer > 0 {
            self.aurora_veil_timer -= 1;
        }
        if self.tailwind_timer > 0 {
            self.tailwind_timer -= 1;
        }
        if self.mist_timer > 0 {
            self.mist_timer -= 1;
        }
        if self.safeguard_timer > 0 {
            self.safeguard_timer -= 1;
        }
    }
    pub fn clear_screens(&mut self) {
        self.reflect_timer = 0;
        self.light_screen_timer = 0;
        self.aurora_veil_timer = 0;
    }
    pub fn stealth_rock(&mut self) {
        self.stealth_rock = true
    }
    pub fn sharp_steel(&mut self) {
        self.sharp_steel = true
    }
    pub fn sticky_web(&mut self) {
        self.sticky_web = true
    }
    pub fn add_spikes(&mut self) {
        self.spikes += 1;
        self.spikes = self.spikes.clamp(0, 3);
    }
    pub fn add_toxic_spikes(&mut self) {
        self.toxic_spikes += 1;
        self.toxic_spikes = self.toxic_spikes.clamp(0, 2);
    }
}

#[derive(Clone)]
pub struct Field {
    pub field_side_a: FieldSide,
    pub field_side_b: FieldSide,
    weather: Weather,
    weather_timer: i32,
    terrain: Terrain,
    terrain_timer: i32,
    gravity_timer: i32,
    trick_room_timer: i32,
    wonder_room_timer: i32,
    magic_room_timer: i32,
}

impl Field {
    pub fn new() -> Self {
        Self {
            field_side_a: FieldSide::new(),
            field_side_b: FieldSide::new(),
            weather: Weather::None,
            weather_timer: 0,
            terrain: Terrain::None,
            terrain_timer: 0,
            gravity_timer: 0,
            trick_room_timer: 0,
            wonder_room_timer: 0,
            magic_room_timer: 0,
        }
    }
    pub fn is_trick_room(&self) -> bool {
        self.trick_room_timer > 0
    }
    pub fn get_weather(&self) -> Weather {
        self.weather
    }
    pub fn get_terrain(&self) -> Terrain {
        self.terrain
    }
    pub fn step_timers(&mut self) {
        self.field_side_a.step_timer();
        self.field_side_b.step_timer();

        if self.weather_timer > 0 {
            self.weather_timer -= 1;
        }
        if self.terrain_timer > 0 {
            self.terrain_timer -= 1;
        }
        if self.gravity_timer > 0 {
            self.gravity_timer -= 1;
        }
        if self.trick_room_timer > 0 {
            self.trick_room_timer -= 1;
        }
        if self.wonder_room_timer > 0 {
            self.wonder_room_timer -= 1;
        }
        if self.magic_room_timer > 0 {
            self.magic_room_timer -= 1;
        }
    }
    pub fn clear_weather(&mut self) {
        if self.weather != Weather::None {
            self.weather = Weather::None
        }
    }
    pub fn clear_terrain(&mut self) {
        if self.terrain != Terrain::None {
            self.terrain = Terrain::None
        }
    }
}
