use anyhow::Result;
use loremaster::data::postgres_handler::PostgresHandler;
use sqlx::PgPool;

const CONNECTION_STRING_ENVIRONMENT_VARIABLE_KEY: &str = "POSTGRES_CONNECTION_STRING";
const DATABASE_FOLDER_PATH: &str = "../database/";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting up...");

    let postgres_connection_string: String =
        std::env::var(CONNECTION_STRING_ENVIRONMENT_VARIABLE_KEY)
            .expect("Missing postgresql connection string.");

    if postgres_connection_string.is_empty() {
        panic!("Postgresql connection string is empty!");
    }

    if !std::path::Path::new(DATABASE_FOLDER_PATH).exists() {
        panic!("Database folder path does not exist!");
    }

    let postgres_handler: PostgresHandler =
        PostgresHandler::new(postgres_connection_string).await?;

    ping(&postgres_handler.database_pool).await?;

    Ok(())
}

pub async fn ping(database_pool: &PgPool) -> Result<()> {
    const PING_QUERY: &str = "SELECT 1;";

    let rows_affected: u64 = sqlx::query(PING_QUERY)
        .execute(database_pool)
        .await?
        .rows_affected();

    println!("{}", rows_affected);

    Ok(())
}
