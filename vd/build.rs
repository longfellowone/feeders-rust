// https://github.com/Trangar/periodic_table/blob/master/build.rs
// https://www.reddit.com/r/rust/comments/f47h5o/include_json_files_along_with_my_library/fhosgxh/?utm_source=share&utm_medium=web2x
// https://docs.rs/serde_cbor/0.11.1/serde_cbor/
// https://stackoverflow.com/questions/50553370/how-do-i-use-include-str-for-multiple-files-or-an-entire-directory/50554062#50554062
// cargo build -vv

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct T2Conductor {
    size: String,
    resistance_75: f64,
}

// https://docs.rs/csv/1.1.3/csv/
fn parse_csv() -> Result<()> {
    let mut rdr = csv::Reader::from_path("test.csv")?;
    // let data = rdr.deserialize().collect::<Result<Vec<Table2Row>, _>>()

    let mut data = BTreeMap::new();

    let mut conductor_sizes = Vec::new();
    let mut conductor_resistances = Vec::new();

    for result in rdr.deserialize() {
        let conductor: T2Conductor = result?;

        conductor_sizes.push(conductor.size.to_string());
        conductor_resistances.push(conductor.resistance_75);

        data.insert(conductor.size.to_string(), conductor);
    }

    let test: BTreeMap<_, _> = conductor_sizes
        .into_iter()
        .zip(conductor_resistances)
        .collect();

    println!("{:#?}", test);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("data.cbor");

    let file = File::create(path)?;
    serde_cbor::to_writer(file, &data)?;

    // AWG sizes (keys) reactance (values) for pvc, Al conduit
    // awg_reactance_pvc_al
    //
    // AWG sizes (keys) reactance (values) for steel conduit
    // awg_reactance_steel
    //
    // AWG sizes (keys) and resistance (values) for CU conductors & pvc conduit
    // cu_resistance_pvc
    //
    // AWG sizes (keys) and resistance (values) for CU conductors & Al conduit
    // cu_resistance_al
    //
    // AWG sizes (keys) and resistance (values) for CU conductors & steel conduit
    // cu_resistance_steel
    //
    // AWG sizes (keys) and resistance (values) for Al conductors & pvc conduit
    // al_resistance_pvc
    //
    // AWG sizes (keys) and resistance (values) for Al conductors & Al conduit
    // al_resistance_al
    //
    // AWG sizes (keys) and resistance (values) for Al conductors & steel conduit
    // al_resistance_steel

    Ok(())
}

fn main() {
    parse_csv().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=test.csv");
}
