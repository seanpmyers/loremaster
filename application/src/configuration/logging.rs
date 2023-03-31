use crate::utility::constants::ENVIRONMENT;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn configure_logging() {
    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "[LOREMASTER_{}]: [{}] [{}] - {}",
                std::env::var(ENVIRONMENT).unwrap().to_ascii_uppercase(),
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
