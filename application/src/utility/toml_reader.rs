use anyhow::{anyhow, Result};
use std::fs;
use toml::Value;

pub fn get_field_value(file_path: &str, field_name: &str) -> Result<String> {
    let string_file_content: String = fs::read_to_string(file_path)?;

    let value_map: Value = string_file_content.parse::<Value>()?;

    if let Some(value) = value_map[field_name].as_str() {
        Ok(value.to_string())
    } else {
        Err(anyhow!("No value found for field name: {}", field_name))
    }
}
