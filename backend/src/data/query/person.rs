pub mod create_person;

#[cfg(test)]
mod tests {
    use anyhow::{Result};
    use chrono::{Utc, offset, DateTime};
    use mobc::Connection;
    use mobc_postgres::PgConnectionManager;

    use crate::{
      data::{
        postgres_handler::PostgresHandler, 
        query::person::create_person::create_person_query
      }, 
      utility::password_encryption::{
        PasswordEncryption, 
        PasswordEncryptionService}
      };

      const TEST_PASSWORD: &str = "testPassword123!";
      const TEST_EMAIL: &str = "testemail@email.com";


    #[tokio::test]
    async fn test_create_person() -> Result<()> {
      let postgres_context: PostgresHandler = PostgresHandler::new().await?;
      let database_connection: Connection<PgConnectionManager<tokio_postgres::NoTls>> = postgres_context.database_pool.get().await?;
      let test_date: DateTime<Utc> = offset::Utc::now();
      
      let hashed_password: String = PasswordEncryptionService::encrypt_password(TEST_PASSWORD)?;

      let _new_person = create_person_query(
        &database_connection, 
        &TEST_EMAIL.to_string(), 
        &hashed_password,
        &test_date).await?
      ;
      
      return Ok(());
    }
}