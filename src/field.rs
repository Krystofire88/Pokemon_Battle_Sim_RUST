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
