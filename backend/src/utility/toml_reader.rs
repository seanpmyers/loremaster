use toml::Value;
use std::fs;
use anyhow::Result;

pub fn get_toml_field_value(file_name: &str, field_name: &str) -> Result<String> {
   let contents : String = fs::read_to_string(file_name).expect("Something went wrong reading the file");
   let file_map =  contents.parse::<Value>()?;
   let field_value : String = file_map[field_name].as_str().unwrap().to_string();
   return Ok(field_value);
}