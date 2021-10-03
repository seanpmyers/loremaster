pub mod create_person;

#[cfg(test)]
mod tests {
   use std::str::FromStr;

    use anyhow::{Result};
    use chrono::{Duration, Local, Utc};
    use mobc::Connection;
    use mobc_postgres::PgConnectionManager;
    use uuid::Uuid;

    use crate::{data::{postgres_handler::PostgresHandler, query::person::create_person::create_person_query}, utility::password_hashing::{PasswordBasedHashEncryption, PasswordEncryptionService}};


    #[tokio::test]
    async fn test_create_person() -> Result<()> {
      let postgres_context: PostgresHandler = PostgresHandler::new().await?;
      let database_connection: Connection<PgConnectionManager<tokio_postgres::NoTls>> = postgres_context.database_pool.get().await?;
      let test_date: chrono::Date<Local> = Local::today();
      let hashed_password: String = PasswordEncryptionService::hash_password("testPassword123!")?;
      create_person_query(&database_connection, &"testemail@email.com".to_string(), &hashed_password,&test_date).await?;
      return Ok(());
    }
}