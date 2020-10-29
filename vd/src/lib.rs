#![allow(dead_code, unused_variables)]
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct T2Row {
    conductor_size: String,
    resistance_75: f64,
}

// ENUMS
// https://doc.rust-lang.org/reference/items/enumerations.html

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

pub struct Panel {
    size: i32,
    voltage: i32,
    parallel_sets: i32,
    number_conductors: i32,
    max_voltage_drop: i32,
    distance: i32,
    // location: Location,
}

pub struct Transformer {
    kva: i32,
    primary_voltage: i32,
    primary_parallel_sets: i32,
    primary_number_conductors: i32,
    secondary_voltage: i32,
    secondary_parallel_sets: i32,
    secondary_number_conductors: i32,
    max_voltage_drop: i32,
    distance: i32,
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
    static ref T2: BTreeMap<String, T2Row> =
        serde_cbor::from_slice(include_bytes!(concat!(env!("OUT_DIR"), "\\data.cbor"))).unwrap();
}

pub fn t2() -> &'static BTreeMap<String, T2Row> {
    &*T2
}

pub fn min_conductor_size(length: i32, voltage: i32, current: i32) -> Metal {
    // calc_voltage_drop(length, voltage, current);
    // Conductor {
    //     copper: Copper::Cu0,
    //     aluminum: Aluminum::Al250,
    // }
    Metal::Copper(CopperSize::Cu0)
}

pub fn calc_voltage_drop(length: i32, voltage: i32, current: i32) -> f64 {
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

    let multiplier = f64::sqrt(3.0); // (3.0_f64).sqrt()
    let pf: f64 = 0.9;
    let resistance = 0.0847;
    let reactance = 0.041;

    multiplier
        * current as f64
        * (resistance * pf / 1000.0 + reactance * pf.acos().sin() / 1000.0)
        * length as f64
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
            t2().get("600mcm"),
            Some(&T2Row {
                conductor_size: "600mcm".to_string(),
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
    fn test_min_conductor_size() {
        assert_eq!(
            min_conductor_size(155, 208, 160),
            Metal::Copper(CopperSizes::Cu0) // Conductor {
                                            //     copper: Copper::Cu0,
                                            //     aluminum: Aluminum::Al250,
                                            // }
        )
    }
}
