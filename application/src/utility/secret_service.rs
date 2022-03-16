use anyhow::{anyhow, Result};

use super::toml_reader::{self};

pub const ITERATIONS_FIELD: &str = "HASH_ITERATIONS";
pub const POSTGRES_TOML_FIELD: &str = "POSTGRESQL";
pub const SITE_SECRET_FIELD: &str = "SITE_SECRET";

const LOREMASTER_FILE_PATH: &str = "./loremaster.toml";

pub fn get_secret(secret_name: &str) -> Result<String> {
    let result: String = toml_reader::get_field_value(LOREMASTER_FILE_PATH, secret_name)
        .map_err(|error| anyhow!("{}", error))?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::get_secret;

    const TEST_SECRET_FIELD: &str = "TestField";
    const TEST_SECRET_VALUE: &str = "TestValue";

    #[test]
    fn secret_exists() -> Result<()> {
        let result: String = get_secret(TEST_SECRET_FIELD)?;
        assert_eq!(result, TEST_SECRET_VALUE);
        Ok(())
    }
}
