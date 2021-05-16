mod utility;
use sqlx::Pool;
use utility::toml_reader;
use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;

const SECRET_FILE_PATH : &str = "./../../../secrets/loremaster.toml";
const TOML_FIELD_NAME : &str = "POSTGRESQL";

#[tokio::main]
async fn main() -> Result<()>{
    println!("LORE MASTER: Starting up...");
    let connection_string : String = toml_reader::get_toml_field_value(SECRET_FILE_PATH, TOML_FIELD_NAME).context(format!("Failed to get connection string from file!"))?;

    let pool: Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;

    return Ok(());
}