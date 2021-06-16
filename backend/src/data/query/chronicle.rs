pub mod current_chronicle;
pub mod create_chronicle;
pub mod update_chronicle;
pub mod delete_chronicle;
pub mod chronicle_by_id;
pub mod chronicle_by_date;


#[cfg(test)]
mod tests {
    use anyhow::Context;
    use chrono::{Duration, Utc};

    use crate::data::{
       postgres_handler::PostgresHandler, 
       query::chronicle::{
         current_chronicle::get_current_chronicle_query,
         create_chronicle::create_chronicle_query
       }
      };

    #[tokio::test]
    async fn test_get_current_chronicle() {
      let postgres_context: PostgresHandler = PostgresHandler::new().await.unwrap();
      let database_connection = postgres_context.database_pool.get().await.unwrap();
      get_current_chronicle_query(&database_connection).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_chronicle() {
      let postgres_context: PostgresHandler = PostgresHandler::new().await.unwrap();
      let database_connection = postgres_context.database_pool.get().await.unwrap();
      let test_date = Utc::today() + Duration::days(7);
      let query_result = create_chronicle_query(&database_connection, &test_date).await.unwrap();
      assert_eq!(test_date, query_result.date_recorded);
    }

    #[tokio::test]
    async fn test_get_chronicle_by_id() {
      let postgres_context: PostgresHandler = PostgresHandler::new().await.unwrap();
      let database_connection = postgres_context.database_pool.get().await.unwrap();
      
      let query_result = get_current_chronicle_query(&database_connection).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_chronicle_by_date() {
      let postgres_context: PostgresHandler = PostgresHandler::new().await.unwrap();
      let database_connection = postgres_context.database_pool.get().await.unwrap();
      let test_date = Utc::today() + Duration::days(7);
      let query_result = get_current_chronicle_query(&database_connection).await.unwrap();
      if let Some(chronicle) = query_result {
         println!("{}", chronicle.date_recorded);
      }
      else {panic!("get was none!")};
    }

    #[tokio::test]
    async fn test_update_chronicle() {
      let postgres_context: PostgresHandler = PostgresHandler::new().await.unwrap();
      let database_connection = postgres_context.database_pool.get().await.unwrap();
      let test_date = Utc::today() + Duration::days(8);
      let query_result = get_current_chronicle_query(&database_connection).await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_chronicle() {
      let postgres_context: PostgresHandler = PostgresHandler::new().await.unwrap();
      let database_connection = postgres_context.database_pool.get().await.unwrap();
      let test_date = Utc::today() + Duration::days(8);
      let query_result = get_current_chronicle_query(&database_connection).await.unwrap();
    }
}