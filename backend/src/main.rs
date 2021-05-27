mod utility;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use sqlx::{Pool, types::chrono::NaiveDate};
use chrono::{Date, Local, Utc};
use utility::toml_reader;
use uuid::Uuid;
use std::io::Write;
use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;

const SECRET_FILE_PATH : &str = "./../../../secrets/loremaster.toml";
const POSTGRES_TOML_FIELD : &str = "POSTGRESQL";
const CURRENT_CHRONICLE_QUERY : &str = 
"
SELECT DISTINCT
    chronicle.id
    , chronicle.date_recorded
FROM
   public.chronicle
WHERE
   chronicle.date_recorded = CURRENT_DATE
;
";

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

    let connection_string : String = toml_reader::get_toml_field_value(SECRET_FILE_PATH, POSTGRES_TOML_FIELD).context(format!("Failed to get connection string from file!"))?;
    info!("LOREMASTER: Connecting to database...");

    let pool: Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;
    info!("LOREMASTER: Connected to database.");
    let query_result = sqlx::query("
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
    } else {
        info!("LOREMASTER: No chronicle found for today. Generating new chronicle.");
        current_chronicle = GenerateChronicle(&pool).await?;
        info!("LOREMASTER: New chronicle created.");
    }
    

    info!("LOREMASTER: Shutting down...");
    return Ok(());
}

pub struct Chronicle {
    id : Uuid,
    date_recorded : Date<Utc>
}

async fn GenerateChronicle(pool: &Pool<sqlx::Postgres>) -> Result<Chronicle> {
    let today = chrono::offset::Utc::today();
    let today2  = sqlx::types::chrono::Utc::today();
    let insert_result = sqlx::query(
        "
        INSERT INTO
            public.chronicle (date_recorded)
        VALUES 
        ($1)
        RETURNING
            id"
    )
    .bind(today2)
    .fetch_one(pool)
    .await?;

    let mut new_chronicle: Chronicle = Chronicle{
        id: Uuid::new_v4(),
        date_recorded: today
    };


    return Ok(new_chronicle);
}