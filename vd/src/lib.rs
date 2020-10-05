// include_str!(concat!(env!("OUT_DIR"), "\\data.cbor"));
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Table2Row {
    conductor_size: String,
    resistance_75: f64,
}

lazy_static! {
    static ref EXAMPLE2: Vec<Table2Row> =
        serde_cbor::from_slice(include_bytes!(concat!(env!("OUT_DIR"), "\\data.cbor"))).unwrap();
}

pub fn return_ref() {
    println!("{:?}", *EXAMPLE2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_ref_works() {
        assert_eq!(return_ref(), ());
    }
}
