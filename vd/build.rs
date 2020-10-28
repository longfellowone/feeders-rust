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

#[derive(Serialize, Deserialize, Debug)]
struct Table2Row {
    conductor_size: String,
    resistance_75: f64,
}

fn parse_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("test.csv")?;
    // let data = rdr.deserialize().collect::<Result<Vec<Table2Row>, _>>()?;

    let mut data = BTreeMap::new();

    for result in rdr.deserialize() {
        let row: Table2Row = result?;
        data.insert(row.conductor_size.to_string(), row);
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("data.cbor");

    let file = File::create(path)?;
    serde_cbor::to_writer(file, &data)?;

    Ok(())
}

fn main() {
    parse_csv().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=test.csv");
}
