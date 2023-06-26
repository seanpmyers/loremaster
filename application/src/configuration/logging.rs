use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::application::LoremasterWebServerConfiguration;

pub fn configure_logging(configuration: &LoremasterWebServerConfiguration) {
    let environment: String = configuration.environment.to_string().to_ascii_uppercase();
    Builder::new()
        .target(Target::Stdout)
        .format(move |buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "[LOREMASTER_{}]: [{}] [{}] - {}",
                environment,
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
