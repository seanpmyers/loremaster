use anyhow::Result;

use super::logging;

pub async fn configure() -> Result<()> {
    logging::configure_logging();

    Ok(())
}
