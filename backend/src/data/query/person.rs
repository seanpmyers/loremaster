pub mod create_person;
pub mod person_by_email_address;
pub mod credential_by_email_address;

#[cfg(test)]
mod tests {
    use anyhow::{Result};
    use mobc::Connection;
    use mobc_postgres::PgConnectionManager;

    use crate::{
      data::{
        postgres_handler::PostgresHandler, 
        query::person::create_person::create_person_query, entity::person::Person
      }, 
      utility::password_encryption::{
        PasswordEncryption, 
        PasswordEncryptionService}
      };

    use super::person_by_email_address::person_by_email_address_query;

      const TEST_PASSWORD: &str = "testPassword123!";
      const TEST_EMAIL: &str = "testemail@email.com";


    #[tokio::test]
    async fn test_create_person() -> Result<()> {
      let postgres_context: PostgresHandler = PostgresHandler::new()
        .await?;
      let database_connection: Connection<PgConnectionManager<tokio_postgres::NoTls>> = 
        postgres_context.database_pool
        .get()
        .await?;
      
      let encrypted_password: String = PasswordEncryptionService::encrypt_password(
        TEST_PASSWORD
      )?;

      let _new_person: Person = create_person_query(
        &database_connection, 
        &TEST_EMAIL.to_string(), 
        &encrypted_password,
        ).await?
      ;
      
      return Ok(());
    }

    #[tokio::test]
    async fn test_person_by_email_address_query() -> Result<()> {
      let postgres_context: PostgresHandler = PostgresHandler::new()
        .await?;
      let database_connection: Connection<PgConnectionManager<tokio_postgres::NoTls>> = 
        postgres_context.database_pool
        .get()
        .await?;
      
      let _query_result: Option<Person> = person_by_email_address_query(
        &database_connection, 
        &TEST_EMAIL.to_string(), 
        ).await?
      ;
      
      return Ok(());
    }
}