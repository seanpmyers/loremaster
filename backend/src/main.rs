use anyhow::{Context, Result};
use chrono::{Date, Local, Utc};
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use mobc::{Connection, Pool};
use mobc_postgres::{PgConnectionManager, tokio_postgres::{self, Config}};
use std::{io::Write, str::FromStr, time::Duration};
use tokio_postgres::NoTls;
mod utility;
use utility::toml_reader;
use uuid::Uuid;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const SECRET_FILE_PATH : &str = "./../../../secrets/loremaster.toml";
const POSTGRES_TOML_FIELD : &str = "POSTGRESQL";


#[tokio::main]
async fn main() -> Result<()>{
    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| -> Result<(), std::io::Error> {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    info!("LOREMASTER: Starting up...");
    info!("LOREMASTER: Getting database connection...");

    let connection_string : String = toml_reader::get_toml_field_value(SECRET_FILE_PATH, POSTGRES_TOML_FIELD).context(format!("Failed to get connection string from file!"))?;
    
    info!("LOREMASTER: Connecting to database...");

    let database_pool = create_database_pool(&connection_string).context(format!("Something went wrong while creating a database pool!"))?;


    let pool: Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;
    info!("LOREMASTER: Connected to database.");
    let query_result = sqlx::query_as::<_, Chronicle>("
    SELECT DISTINCT
        chronicle.id
        , chronicle.date_recorded
    FROM
       public.chronicle
    WHERE
       chronicle.date_recorded = CURRENT_DATE
    ;"
    ).fetch_optional(&pool)
    .await?;
    
    let mut current_chronicle : Chronicle;
    
    if let Some(row) = query_result {
        info!("LOREMASTER: Existing daily chronicle found.");
        println!("{}, {}", row.id, row.date_recorded);
    } else {
        info!("LOREMASTER: No chronicle found for today. Generating new chronicle.");
        current_chronicle = GenerateChronicle(&pool).await?;
        info!("LOREMASTER: New chronicle created.");
    }
    

    info!("LOREMASTER: Shutting down...");
    return Ok(());
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

pub struct Chronicle {
    id : Uuid,
    date_recorded : Date<Utc>
}

async fn GenerateChronicle(pool: &Pool<sqlx::Postgres>) -> Result<Chronicle> {
    let today = chrono::offset::Utc::today();
    let insert_result = sqlx::query(
        "
        INSERT INTO
            public.chronicle (date_recorded)
        VALUES 
        (TO_DATE($1, 'YYYY-MM-DD'))
        RETURNING
            id
        ;"
    )
    .bind(today.to_string())
    .fetch_one(pool)
    .await?;



    //TODO: FIX ID
    let mut new_chronicle: Chronicle = Chronicle{
        id: Uuid::new_v4(),
        date_recorded: today
    };


    return Ok(new_chronicle);
}