use anyhow::{anyhow, Result};
use sqlx::{pool::PoolOptions, postgres::PgPoolOptions, PgPool};
use std::time::Duration;

const DB_POOL_MAX_OPEN: u32 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

#[derive(Clone)]
pub struct PostgresHandler {
    pub connection_string: String,
    pub database_pool: PgPool,
}

impl PostgresHandler {
    pub async fn new(connection_string: String) -> Result<Self> {
        let database_pool: PgPool = create_database_pool(&connection_string)
            .await
            .map_err(|error| anyhow!("{}", error))?;

        let new_handler: PostgresHandler = PostgresHandler {
            connection_string,
            database_pool,
        };

        Ok(new_handler)
    }
}

pub async fn create_database_pool(connection_string: &str) -> Result<PgPool> {
    let options: PoolOptions<sqlx::Postgres> = PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(DB_POOL_MAX_IDLE))
        .max_connections(DB_POOL_MAX_OPEN)
        .acquire_timeout(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS));

    let result: PgPool = options
        .connect(connection_string)
        .await
        .map_err(|error| anyhow!("{}", error))?;

    Ok(result)
}
