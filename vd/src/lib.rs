#![allow(dead_code, unused_variables)]
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Naming conventions
// https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

// ENUMS
// https://doc.rust-lang.org/reference/items/enumerations.html

// Use match on metal/phase/conduit/unit
// vd::maxdistance::new() ::minconductor
// calc::from(panel)
// vd::single_phase ::three_phase -> crate::vd
// vd::dc

// References
// http://profwagner.com/4520/4520-PPT10.pdf
// https://pdhonline.com/courses/e426/e426content.pdf
// https://github.com/MasonMcGarrity/Voltage_Drop_Calculator/blob/master/main.py#L282
// https://github.com/Zclarkwilliams/Voltage-Drop-Excel-Calculator/blob/master/Code/Main_Rev2.vba

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct T2Conductor {
    size: String,
    resistance_75: f64,
}
// pub struct Location {
//     source: String,
//     destination: String,
// }

#[derive(Debug, PartialEq)]
pub enum Metal {
    Copper(CopperSize),
    Aluminum(AluminumSize),
}

// #[derive(Debug, PartialEq)]
// pub struct Conductor {
//     copper: Copper,
//     aluminum: Aluminum,
// }

#[derive(Debug, PartialEq)]
pub enum CopperSize {
    Cu12,
    Cu10,
    Cu8,
    Cu6,
    Cu0,
    Cu00,
    Cu000,
    Cu250,
}

#[derive(Debug, PartialEq)]
pub enum AluminumSize {
    Al250,
}

// Note: Bond size must increase when conductor size is increased due to VD

pub struct Motor {
    voltage: i32,
    hp: i32,
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
    // location: Location,
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
    // distance: i32,
    // location: Location,
}

pub struct Pull {
    conductor_size: String,
    voltage_drop: i32,
    termination_temperature: i32,
    // location: Location,
    // ground_size: String,
}

// pub type FeederSchedule = Vec<Pull>;

lazy_static! {
    static ref T2: BTreeMap<String, T2Conductor> =
        serde_cbor::from_slice(include_bytes!(concat!(env!("OUT_DIR"), "\\data.cbor"))).unwrap();
}

// Maybe use approximate than double check?
// https://pdhonline.com/courses/e426/e426content.pdf see page49/57
// Or just use vd function recursively until match
// https://stackoverflow.com/questions/49599833/how-to-find-next-smaller-key-in-btreemap-btreeset
// https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
pub fn min_conductor_size(length_ft: i32, voltage: i32, current: i32) -> Metal {
    // calc_voltage_drop(length, voltage, current);
    // Conductor {
    //     copper: Copper::Cu0,
    //     aluminum: Aluminum::Al250,
    // }
    Metal::Copper(CopperSize::Cu0)
}

// Estimated, does not account for error
// https://pdhonline.com/courses/e426/e426content.pdf see page 36/57
pub fn calc_voltage_drop(length_ft: i32, voltage: i32, current: i32) -> f64 {
    // https://www.southwire.com/calculator-vdrop

    // calctype: "minConductorSize"
    // conductorSize: "Conductor Size"
    // current: "12"
    // installation: "directBuried"
    // length: "100"
    // maxVoltage: "3"
    // metal: "copper"
    // phase: "one"
    // sets: "1"
    // units: "imperial"
    // voltage: "120"

    // Single phase use 2 instead of sqrt3

    let power_factor: f64 = 0.9; // PF of 0.85 most common
    let theta = f64::acos(power_factor);
    let multiplier = f64::sqrt(3.0); // (3.0_f64).sqrt() for line-to-line voltage drop, Multiply for 2 instead for line-to-neutral
    let resistance = 0.0847; // R
    let reactance = 0.041; // Xl
    let impedance = resistance * theta.cos() + reactance * theta.sin(); // Effective Z, Addition based on assumed lagging PF
    let vd = current as f64 * impedance / 1000.0 * length_ft as f64;

    multiplier * vd
}

pub fn calc_change_in_resistance(from: i32, to: i32, resistance: f64) -> f64 {
    let a = 0.00323; // Copper
    resistance * (1.0 + a * (to as f64 - from as f64))
}

pub fn calc_resistance_required() -> f64 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t2_has_data() {
        assert_eq!(
            T2.get("600mcm"),
            Some(&T2Conductor {
                size: "600mcm".to_string(),
                resistance_75: 0.0214
            })
        );
    }

    #[test]
    fn test_calc_voltage_drop() {
        assert_eq!(calc_voltage_drop(155, 208, 160), 4.042116145290523)
    }

    #[test]
    fn test_calc_resistance_required() {
        assert_eq!(calc_resistance_required(), 0.0)
    }

    #[test]
    fn test_calc_change_in_resistance() {
        assert_eq!(calc_change_in_resistance(75, 20, 1.2), 0.98682)
    }
    #[test]
    fn test_min_conductor_size() {
        assert_eq!(
            min_conductor_size(155, 208, 160),
            // Conductor {
            //     copper: Copper::Cu0,
            //     aluminum: Aluminum::Al250,
            // }
            Metal::Copper(CopperSize::Cu0)
        )
    }
}
