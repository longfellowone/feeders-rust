// https://github.com/Trangar/periodic_table/blob/master/build.rs
// https://www.reddit.com/r/rust/comments/f47h5o/include_json_files_along_with_my_library/fhosgxh/?utm_source=share&utm_medium=web2x
// https://docs.rs/serde_cbor/0.11.1/serde_cbor/
// https://stackoverflow.com/questions/50553370/how-do-i-use-include-str-for-multiple-files-or-an-entire-directory/50554062#50554062

use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Table2Row {
    conductor_size: String,
    resistance_75: f64,
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut table2 = vec![];

    let mut rdr = csv::Reader::from_path("test.csv").unwrap();

    for row in rdr.deserialize() {
        let task: Table2Row = row?;
        table2.push(task);
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");
    let _file = File::create(dest_path).unwrap();

    println!("{:#?}", table2);

    Ok(())
}

fn main() {
    read_csv().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=test.csv");
}
