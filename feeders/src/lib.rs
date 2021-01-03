#![allow(dead_code, unused_variables)]
// Note: Bond size must increase when conductor size is increased due to VD

// Newtype
// https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html

// Naming conventions
// https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

// ENUMS
// https://doc.rust-lang.org/reference/items/enumerations.html

// vd::MaxDistance::from(panel)

// Use try_into()

pub type FeederSchedule = Vec<Pull>;

pub struct Pull {
    conductor_size: String,
    voltage_drop: i32,
    termination_temperature: i32,
    location: Location,
    ground_size: String,
}

pub struct Location {
    source: String,
    destination: String,
}

pub struct Panel {
    size: i32,
    voltage: i32,
    parallel_sets: i32,
    number_conductors: i32,
    termination_temperature: i32,
    overcurrent_size: i32,
    // max_voltage_drop: i32,
    // distance: i32,
    location: Location,
}

pub struct Transformer {
    kva: i32,
    primary_voltage: i32,
    primary_parallel_sets: i32,
    primary_number_conductors: i32,
    primary_termination_temperature: i32,
    primary_overcurrent_size: i32,
    secondary_voltage: i32,
    secondary_parallel_sets: i32,
    secondary_number_conductors: i32,
    // max_voltage_drop: i32,
    distance: i32,
    location: Location,
}

pub struct Motor {
    voltage: i32,
    hp: i32,
}
