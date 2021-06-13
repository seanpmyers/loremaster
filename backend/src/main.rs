use anyhow::{Context, Result};
use chrono::{Date, Local, TimeZone, Utc};
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use mobc::{Connection, Pool};
use mobc_postgres::{PgConnectionManager, tokio_postgres::{self, Config}};
use core::str;
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

const CURRENT_CHRONICLE_QUERY : &str = "
    SELECT DISTINCT
        chronicle.id
        , chronicle.date_recorded
    FROM
        public.chronicle
    WHERE
        chronicle.date_recorded = CURRENT_DATE
    ;";

// const ALL_CHRONICLES_QUERY : &str = "
//     SELECT
//         *
//     FROM
//         public.chronicle
//     LIMIT 1
//     ;";

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

    info!("LOREMASTER: Connected to database.");

    let database_connection = database_pool.get().await.context(format!("Failed to get database connection!"))?;
    
    info!("LOREMASTER: Querying for today's chronicle.");

    let query_result = get_current_chronicle_query(&database_connection).await.context(format!("Failed to execute query for current chronicle!"))?;
    
    match query_result {
        Some(chronicle_result) => {
            info!("LOREMASTER: Existing chronicle found!");
            println!("{}, {}", chronicle_result.id, chronicle_result.date_recorded);
        }
        None => {
            info!("LOREMASTER: No chronicle found for today. Generating new chronicle...");
            let chronicle_result = generate_chronicle(&&database_connection).await.context(format!("Failed to execute create new chronicle query!"))?;
            println!("{}, {}", chronicle_result.id, chronicle_result.date_recorded);
        }
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

#[derive(Debug)]
pub struct Chronicle {
    id : Uuid,
    date_recorded : Date<Utc>
}

const NEW_CHRONICLE_QUERY : &str = "
    INSERT INTO
        public.chronicle (date_recorded)
    VALUES 
    (TO_DATE($1, 'YYYY-MM-DD'))
    RETURNING
        id
    ;";

async fn generate_chronicle(database_connection: &Connection<PgConnectionManager<NoTls>>) -> Result<Chronicle> {
    let today = chrono::offset::Utc::today();
    let query_result = database_connection.query_one(NEW_CHRONICLE_QUERY, &[&today.to_string()]).await.context(format!("An error occurred while querying the database."))?;
    let result_id: Uuid = query_result.get::<_, Uuid>("id");

    let new_chronicle: Chronicle = Chronicle{
        id: result_id,
        date_recorded: today
    };

    return Ok(new_chronicle);
}

async fn get_current_chronicle_query(database_connection: &Connection<PgConnectionManager<NoTls>>) -> Result<Option<Chronicle>> {
    let query_result = database_connection.query(CURRENT_CHRONICLE_QUERY, &[]).await.context(format!("An error occurred while querying the database."))?;
    if query_result.len() == 0 { return Ok(None);}
    
    match query_result.get(0) {
        Some(chronicle_result) => {
            let result = Chronicle {
                id: chronicle_result.get::<_, Uuid>("id"),
                date_recorded: Utc.from_utc_date(&chronicle_result.get::<_, chrono::NaiveDate>("date_recorded")) 
            };
            return Ok(Some(result));
        }
        None => {
            return Ok(None);
        }
    }
}