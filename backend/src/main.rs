mod utility;
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use sqlx::Pool;
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

    let thing = sqlx::query("
    SELECT DISTINCT
        chronicle.id
        , chronicle.date_recorded
    FROM
       public.chronicle
    WHERE
       chronicle.date_recorded = CURRENT_DATE
    ;"
    ).execute(&pool)
    .await?;

    info!("LOREMASTER: Shutting down...");
    return Ok(());
}

pub struct chronicle {
    id : Uuid,
    date_recorded : Date<Utc>
}