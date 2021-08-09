pub mod create_person;

#[cfg(test)]
mod tests {
   use std::str::FromStr;

    use anyhow::{Result};
    use chrono::{Duration, Local, Utc};
    use uuid::Uuid;

    use crate::data::{postgres_handler::PostgresHandler, query::person::create_person::create_person_query};


    #[tokio::test]
    async fn test_create_person() -> Result<()> {
      let postgres_context: PostgresHandler = PostgresHandler::new().await?;
      let database_connection = postgres_context.database_pool.get().await?;
      let test_date = Local::today();
      let query_result = create_person_query(&database_connection, &"testemail@email.com".to_string(), &"".to_string(),&test_date).await?;
      assert_eq!(test_date, query_result.date_recorded);
      return Ok(());
    }
}