mod utility;

use utility::toml_reader;
use anyhow::{Context, Result};

const SECRET_FILE_PATH : &str = "./../../../secrets/loremaster.toml";
const TOML_FIELD_NAME : &str = "POSTGRESQL";

fn main() {
    println!("LORE MASTER: Starting up...");
    let connection_string : String = toml_reader::get_toml_field_value(SECRET_FILE_PATH, TOML_FIELD_NAME).unwrap();
    println!("{}", connection_string);
}
