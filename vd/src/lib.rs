use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Table2Row {
    conductor_size: String,
    resistance_75: f64,
}

lazy_static! {
    static ref EXAMPLE2: Vec<Table2Row> =
        serde_cbor::from_slice(include_bytes!(concat!(env!("OUT_DIR"), "\\data.cbor"))).unwrap();
}

pub fn table2() -> &'static Vec<Table2Row> {
    &*EXAMPLE2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table2_has_data() {
        assert_eq!(
            table2().into_iter().next(),
            Some(&Table2Row {
                conductor_size: "600mcm".to_string(),
                resistance_75: 0.0214
            })
        );
    }
}
