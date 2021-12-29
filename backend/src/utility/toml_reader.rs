use anyhow::{anyhow, Context, Result};
use std::fs;
use toml::Value;

pub fn get_field_value(file_name: &str, field_name: &str) -> Result<String> {
   let string_file_content : String = fs::read_to_string(file_name)
      .context(format!("Something went wrong reading the file"))?;
   let value_map: Value =  string_file_content.parse::<Value>()
      .context(format!("Something went wrong while parsing toml file's string content!"))?;
   if let Some(value) = value_map[field_name].as_str() {
      return Ok(value.to_string())
   } else {
      return Err(anyhow!("No value found for field name: {}", field_name))
   };
}