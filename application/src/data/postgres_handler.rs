use anyhow::{anyhow, Result};
use mobc::Pool;
use mobc_postgres::{
    tokio_postgres::{self, Config},
    PgConnectionManager,
};
use std::{str::FromStr, time::Duration};
use tokio_postgres::NoTls;

use crate::utility::secret_service::{get_secret, POSTGRES_TOML_FIELD};

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub struct PostgresHandler {
    pub connection_string: String,
    pub database_pool: Pool<PgConnectionManager<NoTls>>,
}

impl PostgresHandler {
    pub async fn new() -> Result<Self> {
        let connection_string: String =
            get_secret(POSTGRES_TOML_FIELD).map_err(|error| anyhow!("{}", error))?;

        let database_pool: Pool<PgConnectionManager<NoTls>> =
            create_database_pool(&connection_string).map_err(|error| anyhow!("{}", error))?;

        let new_handler: PostgresHandler = PostgresHandler {
            connection_string,
            database_pool,
        };

        Ok(new_handler)
    }
}

pub fn create_database_pool(connection_string: &str) -> Result<Pool<PgConnectionManager<NoTls>>> {
    let config: Config =
        Config::from_str(connection_string).map_err(|error| anyhow!("{}", error))?;

    let manager: PgConnectionManager<NoTls> = PgConnectionManager::new(config, NoTls);

    let result: Pool<PgConnectionManager<NoTls>> = Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager);

    Ok(result)
}
