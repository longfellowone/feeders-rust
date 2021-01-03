#![allow(dead_code, unused_variables)]
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Use match on metal/phase/conduit/unit

// vd::single_phase ::three_phase -> crate::vd
// vd::dc

// References
// http://profwagner.com/4520/4520-PPT10.pdf
// https://pdhonline.com/courses/e426/e426content.pdf
// https://github.com/MasonMcGarrity/Voltage_Drop_Calculator/blob/master/main.py#L282
// https://github.com/Zclarkwilliams/Voltage-Drop-Excel-Calculator/blob/master/Code/Main_Rev2.vba

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct T2Conductor {
    pub size: String,
    pub resistance_75: f64,
}

#[derive(Debug, PartialEq)]
pub struct Conductor {
    size: String,
    metal: Metal,
}

#[derive(Debug, PartialEq)]
pub enum Metal {
    Copper,
    Aluminum,
}

#[derive(Debug, PartialEq)]
pub enum ConduitType {
    Steel,
    PVC,
    Aluminum,
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    Imperial,
    Metric,
}

#[derive(Debug, PartialEq)]
pub enum Phase {
    Single,
    Three,
}

#[derive(Debug, PartialEq)]
pub enum VoltageType {
    AC,
    DC,
}

// Have separate for DC?
#[derive(Debug, PartialEq)]
pub struct MinConductorSize {
    pub voltage: i32,
    pub current: i32,
    pub length: i32,
    pub max_vd_percentage: f64,
    pub temperature: i32,
    pub power_factor: f64,
    pub voltage_type: VoltageType,
    pub phase: Phase,
    pub conduit_type: ConduitType,
    pub metal: Metal,
    pub unit: Unit,
}

impl MinConductorSize {
    pub fn new() -> Self {
        MinConductorSize {
            voltage: 0,
            current: 0,
            length: 0,
            phase: Phase::Single,
            max_vd_percentage: 0.0,
            conduit_type: ConduitType::Steel,
            metal: Metal::Copper,
            unit: Unit::Imperial,
            power_factor: 85.0,
            temperature: 75,
            voltage_type: VoltageType::AC,
        }
    }

    pub fn calculate(self: Self) -> Conductor {
        Conductor {
            size: "250".to_string(),
            metal: Metal::Copper,
        }
    }
}

pub struct MaxDistance {}

lazy_static! {
    pub static ref T2: BTreeMap<String, T2Conductor> =
        serde_cbor::from_slice(include_bytes!(concat!(env!("OUT_DIR"), "\\data.cbor"))).unwrap();
}

// Maybe use approximate than double check?
// https://pdhonline.com/courses/e426/e426content.pdf see page49/57
// Or just use vd function recursively until match
// https://stackoverflow.com/questions/49599833/how-to-find-next-smaller-key-in-btreemap-btreeset
// https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
pub fn min_conductor_size(length_ft: i32, voltage: i32, current: i32) -> Conductor {
    Conductor {
        size: "500mcm".to_string(),
        metal: Metal::Copper,
    }
}

// Estimated, does not account for error
// https://pdhonline.com/courses/e426/e426content.pdf see page 36/57
pub fn voltage_drop_ac(length_ft: i32, voltage: i32, current: i32) -> f64 {
    let power_factor: f64 = 0.9; // PF of 0.85 most common
    let theta = f64::acos(power_factor); // Power factor angle
    let multiplier = f64::sqrt(3.0); // (3.0_f64).sqrt() for line-to-line voltage drop, Multiply for 2 instead for line-to-neutral
    let resistance = 0.0847; // R
    let reactance = 0.041; // X
    let impedance = resistance * power_factor + reactance * theta.sin(); // Effective Z, Addition based on assumed lagging PF

    multiplier * current as f64 * impedance * length_ft as f64 / 1000.0
}

pub fn voltage_drop_dc(length_ft: i32, voltage: i32, current: i32) -> f64 {
    let resistance = 0.0847; // R
    let impedance = resistance;

    current as f64 * impedance * length_ft as f64 / 1000.0
}

pub fn calc_resistance_required() -> f64 {
    0.0
}

fn modify_resistance_temperature(resistance: f64, to_temperature: i32) -> f64 {
    const FROM_TEMPERATURE: f64 = 75.0; // Table 9 resistance values based of 75 degrees
    const A: f64 = 0.00323; // Temperature coefficient of copper @ 75 degrees. Aluminum 0.00330

    resistance * (1.0 + A * (to_temperature as f64 - FROM_TEMPERATURE))
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
    fn test_min_conductor_size_calculate() {
        let min_conductor_size = MinConductorSize::new();
        assert_eq!(
            min_conductor_size.calculate(),
            Conductor {
                size: "250".to_string(),
                metal: Metal::Copper
            }
        )
    }

    #[test]
    fn test_calc_voltage_drop() {
        assert_eq!(voltage_drop_ac(155, 208, 160), 4.042116145290523)
    }

    #[test]
    fn test_voltage_drop_dc() {}

    #[test]
    fn test_calc_resistance_required() {
        assert_eq!(calc_resistance_required(), 0.0)
    }

    #[test]
    fn test_modify_resistance_temperature() {
        assert_eq!(modify_resistance_temperature(1.2, 20), 0.98682)
    }
    #[test]
    fn test_min_conductor_size() {
        assert_eq!(
            min_conductor_size(155, 208, 160),
            Conductor {
                size: "500mcm".to_string(),
                metal: Metal::Copper
            }
        )
    }
}
