use crate::enums::*;

#[derive(Clone)]
pub struct FieldSide {
    stealth_rock: bool,
    sharp_steel: bool,
    sticky_web: bool,
    spikes: i32,
    toxic_spikes: i32,
    reflect: bool,
    reflect_timer: i32,
    light_screen: bool,
    light_screen_timer: i32,
    aurora_veil: bool,
    aurora_veil_timer: i32,
    tailwind: bool,
    tailwind_timer: i32,
    mist: bool,
    mist_timer: i32,
}

impl FieldSide {
    pub fn new() -> Self {
        Self {
            stealth_rock: false,
            sharp_steel: false,
            sticky_web: false,
            spikes: 0,
            toxic_spikes: 0,
            reflect: false,
            reflect_timer: 0,
            light_screen: false,
            light_screen_timer: 0,
            aurora_veil: false,
            aurora_veil_timer: 0,
            tailwind: false,
            tailwind_timer: 0,
            mist: false,
            mist_timer: 0,
        }
    }
}

#[derive(Clone)]
pub struct Field {
    field_side_a: FieldSide,
    field_side_b: FieldSide,
    weather: Weather,
    weather_timer: i32,
    terrain: Terrain,
    terrain_timer: i32,
    gravity: bool,
    gravity_timer: i32,
    trick_room: bool,
    trick_room_timer: i32,
    wonder_room: bool,
    wonder_room_timer: i32,
    magic_room: bool,
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
            gravity: false,
            gravity_timer: 0,
            trick_room: false,
            trick_room_timer: 0,
            wonder_room: false,
            wonder_room_timer: 0,
            magic_room: false,
            magic_room_timer: 0,
        }
    }
}
