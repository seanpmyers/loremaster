use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

use super::constants::LOREMASTER_CONFIGURATION_FILE_PATH;

#[derive(Clone, Deserialize, Serialize)]
pub struct LoremasterConfiguration {
    pub test_field: String,
    pub postgresql_connection_string: String,
    pub port: u16,
    pub ipv4_address: [u8; 4],
    pub hash_iterations: u32,
    pub site_secret: String,
    pub frontend_port: u16,
}

#[derive(Deserialize, Serialize)]
struct LoremasterConfigurationFile {
    pub test_field: String,
    pub local: Local,
    pub dev: Dev,
    pub qa: QA,
    pub prod: Prod,
}

#[derive(Deserialize, Serialize)]
struct Local {
    pub database: Database,
    pub encryption: Encryption,
    pub web_server: WebServer,
    pub frontend: Frontend,
}

#[derive(Deserialize, Serialize)]
struct Dev {
    pub database: Database,
    pub encryption: Encryption,
    pub web_server: WebServer,
    pub frontend: Frontend,
}

#[derive(Deserialize, Serialize)]
struct QA {
    pub database: Database,
    pub encryption: Encryption,
    pub web_server: WebServer,
    pub frontend: Frontend,
}

#[derive(Deserialize, Serialize)]
struct Prod {
    pub database: Database,
    pub encryption: Encryption,
    pub web_server: WebServer,
    pub frontend: Frontend,
}

#[derive(Deserialize, Serialize)]
struct Database {
    pub postgresql_connection_string: String,
}

#[derive(Deserialize, Serialize)]
struct Encryption {
    pub hash_iterations: i64,
    pub site_secret: String,
}

#[derive(Deserialize, Serialize)]
struct WebServer {
    pub port: i64,
    pub ipv4_address: [i64; 4],
}

#[derive(Deserialize, Serialize)]
struct Frontend {
    pub port: i64,
}

pub fn get_configuration_from_file(environment: &String) -> Result<LoremasterConfiguration> {
    let file_content: String = fs::read_to_string(LOREMASTER_CONFIGURATION_FILE_PATH)?;

    let configuration: LoremasterConfigurationFile = toml::from_str(&file_content)?;

    match environment.as_str() {
        "local" => Ok(LoremasterConfiguration {
            test_field: configuration.test_field,
            postgresql_connection_string: configuration.local.database.postgresql_connection_string,
            port: configuration.local.web_server.port as u16,
            ipv4_address: convert_i64_vec_to_u8(configuration.local.web_server.ipv4_address)?,
            hash_iterations: configuration.local.encryption.hash_iterations as u32,
            site_secret: configuration.local.encryption.site_secret,
            frontend_port: configuration.local.frontend.port as u16,
        }),
        "dev" => Ok(LoremasterConfiguration {
            test_field: configuration.test_field,
            postgresql_connection_string: configuration.dev.database.postgresql_connection_string,
            port: configuration.dev.web_server.port as u16,
            ipv4_address: convert_i64_vec_to_u8(configuration.dev.web_server.ipv4_address)?,
            hash_iterations: configuration.dev.encryption.hash_iterations as u32,
            site_secret: configuration.dev.encryption.site_secret,
            frontend_port: configuration.dev.frontend.port as u16,
        }),
        "qa" => Ok(LoremasterConfiguration {
            test_field: configuration.test_field,
            postgresql_connection_string: configuration.qa.database.postgresql_connection_string,
            port: configuration.qa.web_server.port as u16,
            ipv4_address: convert_i64_vec_to_u8(configuration.qa.web_server.ipv4_address)?,
            hash_iterations: configuration.qa.encryption.hash_iterations as u32,
            site_secret: configuration.qa.encryption.site_secret,
            frontend_port: configuration.qa.frontend.port as u16,
        }),
        "prod" => Ok(LoremasterConfiguration {
            test_field: configuration.test_field,
            postgresql_connection_string: configuration.prod.database.postgresql_connection_string,
            port: configuration.prod.web_server.port as u16,
            ipv4_address: convert_i64_vec_to_u8(configuration.prod.web_server.ipv4_address)?,
            hash_iterations: configuration.prod.encryption.hash_iterations as u32,
            site_secret: configuration.prod.encryption.site_secret,
            frontend_port: configuration.prod.frontend.port as u16,
        }),
        _ => panic!("Invalid environment!"),
    }
}

fn convert_i64_vec_to_u8(address: [i64; 4]) -> Result<[u8; 4]> {
    let mut result: [u8; 4] = [0_u8; 4];
    for (index, value) in address.iter().enumerate() {
        result[index] = value.to_owned() as u8;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{get_configuration_from_file, LoremasterConfiguration};

    const TEST_SECRET_VALUE: &str = "TestValue";

    #[test]
    fn test_field_exists() -> Result<()> {
        let result: LoremasterConfiguration = get_configuration_from_file(&String::from("dev"))?;
        assert_eq!(result.test_field, TEST_SECRET_VALUE);
        Ok(())
    }

    #[test]
    fn environment_test() -> Result<()> {
        let result: LoremasterConfiguration = get_configuration_from_file(&String::from("dev"))?;
        assert_eq!(result.port, 8000);
        Ok(())
    }
}
