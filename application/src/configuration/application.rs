use std::{fmt, fs};

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};

use crate::utility::constants::LOREMASTER_CONFIGURATION_FILE_PATH;

use super::logging;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoremasterWebServerConfiguration {
    pub environment: Environment,
    pub web_server: WebServer,
    pub database: Database,
    pub encryption: Encryption,
    pub front_end: FrontEnd,
    pub scheduler_state: SchedulerState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Local,
    Development,
    QualityAssurance,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulerState {
    On,
    Off,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebServer {
    pub port: u16,
    pub ipv4_address: [u8; 4],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Database {
    pub postgresql_connection_string: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Encryption {
    pub hash_iterations: u32,
    pub site_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrontEnd {
    pub port: u16,
}

pub async fn configure() -> Result<LoremasterWebServerConfiguration> {
    info!(
        "Reading configuration from `{}` file.",
        LOREMASTER_CONFIGURATION_FILE_PATH
    );
    let file_content: String = fs::read_to_string(LOREMASTER_CONFIGURATION_FILE_PATH)?;
    let configuration: LoremasterWebServerConfiguration = ron::from_str(&file_content)?;
    logging::configure_logging(&configuration);

    Ok(configuration)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::LoremasterWebServerConfiguration;

    const TEST_RON_CONTENT: &str = r#"
    LoremasterWebServerConfiguration(
        environment: Local,
        scheduler_state: Off,
        web_server: (
            port: 8000,
            ipv4_address: (127, 0, 0, 1),
        ),
        database: (
            postgresql_connection_string: "postgres://postgres:postgres@localhost/postgres",
        ),
        encryption: (
            hash_iterations: 2,
            site_secret: "replace with a secret",
        ),
        front_end: (
            port: 8080
        ),
    )
    "#;

    #[test]
    fn environment_test() -> Result<()> {
        let configuration: LoremasterWebServerConfiguration = ron::from_str(TEST_RON_CONTENT)?;
        assert_eq!(
            configuration.web_server.port, 8000,
            "Test web server port: {} {}",
            configuration.web_server.port, 8000
        );
        Ok(())
    }
}
