use anyhow::{Context, Result};

use super::toml_reader::{self};

pub const ITERATIONS_FIELD : &str = "HASH_ITERATIONS";
pub const POSTGRES_TOML_FIELD : &str = "POSTGRESQL";
pub const SITE_SECRET_FIELD : &str = "SITE_SECRET";

const SECRET_FILE_PATH : &str = "./../../../secrets/loremaster.toml";

pub fn get_secret(secret_name: &str) -> Result<String> {
   let result = toml_reader::get_field_value(SECRET_FILE_PATH, secret_name)
      .context("Requested secret not available in secret file!".to_string())?;
   return Ok(result);
}