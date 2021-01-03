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

fn main() {
    parse_csv().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=test.csv");
}

#[derive(Serialize, Deserialize, Debug)]
struct Table9 {
    reactance_pvc_al: BTreeMap<String, f64>,
    reactance_steel: BTreeMap<String, f64>,
    cu_resistance_pvc: BTreeMap<String, f64>,
    cu_resistance_al: BTreeMap<String, f64>,
    cu_resistance_steel: BTreeMap<String, f64>,
    al_resistance_pvc: BTreeMap<String, f64>,
    al_resistance_al: BTreeMap<String, f64>,
    al_resistance_steel: BTreeMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Table9Row {
    size: String,
    reactance_pvc_al: f64,
    reactance_steel: f64,
    cu_resistance_pvc: f64,
    cu_resistance_al: f64,
    cu_resistance_steel: f64,
    al_resistance_pvc: f64,
    al_resistance_al: f64,
    al_resistance_steel: f64,
}

fn parse_csv() -> Result<()> {
    let mut rdr = csv::Reader::from_path("data/nec-table9.csv")?;

    let mut t9 = Table9 {
        reactance_pvc_al: Default::default(),
        reactance_steel: Default::default(),
        cu_resistance_pvc: Default::default(),
        cu_resistance_al: Default::default(),
        cu_resistance_steel: Default::default(),
        al_resistance_pvc: Default::default(),
        al_resistance_al: Default::default(),
        al_resistance_steel: Default::default(),
    };

    for result in rdr.deserialize() {
        let row: Table9Row = result?;

        t9.reactance_pvc_al
            .insert(row.size.to_owned(), row.reactance_pvc_al);
        t9.reactance_steel
            .insert(row.size.to_owned(), row.reactance_steel);

        t9.cu_resistance_pvc
            .insert(row.size.to_owned(), row.cu_resistance_pvc);
        t9.cu_resistance_al
            .insert(row.size.to_owned(), row.cu_resistance_al);
        t9.cu_resistance_steel
            .insert(row.size.to_owned(), row.cu_resistance_steel);

        t9.al_resistance_pvc
            .insert(row.size.to_owned(), row.al_resistance_pvc);
        t9.al_resistance_al
            .insert(row.size.to_owned(), row.al_resistance_al);
        t9.al_resistance_steel
            .insert(row.size.to_owned(), row.al_resistance_steel);
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("t9.cbor");

    let file = File::create(path)?;
    serde_cbor::to_writer(file, &t9)?;

    Ok(())
}
