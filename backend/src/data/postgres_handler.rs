use anyhow::{Context, Result};
use mobc::{Pool};
use mobc_postgres::{PgConnectionManager, tokio_postgres::{self, Config}};
use std::{str::FromStr, time::Duration};
use tokio_postgres::NoTls;

use crate::utility::secret_service::{get_secret, POSTGRES_TOML_FIELD};

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub struct PostgresHandler {
   pub connection_string: String,
   pub database_pool: Pool<PgConnectionManager<NoTls>>
}

impl PostgresHandler {
   pub async fn new() -> Result<Self> {
      let connection_string : String = get_secret(POSTGRES_TOML_FIELD)
         .context(format!("Failed to get connection string from secret file!"))?;
      let database_pool = create_database_pool(&connection_string)
         .context(format!("Something went wrong while creating a database pool!"))?; 
      let new_handler: PostgresHandler = PostgresHandler{connection_string: connection_string, database_pool};
      return Ok(new_handler);
   }
}


pub fn create_database_pool(connection_string: &str) -> Result<Pool<PgConnectionManager<NoTls>>> {
   let config = Config::from_str(connection_string).context(format!("Failed to create database config from connection string!"))?;
   let manager = PgConnectionManager::new(config, NoTls);
   let result = Pool::builder()
      .max_open(DB_POOL_MAX_OPEN)
      .max_idle(DB_POOL_MAX_IDLE)
      .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
      .build(manager);
   return Ok(result);
}
