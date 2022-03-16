pub mod chronicle_by_date;
pub mod chronicle_by_id;
pub mod create_chronicle;
pub mod current_chronicle;
pub mod current_chronicle_by_person;
pub mod delete_chronicle;
pub mod update_chronicle;

//TODO: Change test date to be in the past
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use chrono::{offset, DateTime, Duration, Utc};
    use uuid::Uuid;

    use crate::data::{
        entity::chronicle::Chronicle,
        postgres_handler::PostgresHandler,
        query::chronicle::{
            chronicle_by_date::chronicle_by_date_query, chronicle_by_id::chronicle_by_id_query,
            create_chronicle::create_chronicle_query,
            current_chronicle_by_person::get_current_chronicle_by_person_query,
            delete_chronicle::delete_chronicle_query, update_chronicle::update_chronicle_query,
        },
    };

    const TEST_PERSON_ID: &str = "";
    const TEST_CHRONICLE_ID: &str = "98e94305-78c6-44f7-85fa-33f485647f7e";

    #[tokio::test]
    async fn test_get_current_chronicle() -> Result<()> {
        let postgres_context: PostgresHandler = PostgresHandler::new().await?;
        let database_connection = postgres_context.database_pool.get().await?;
        get_current_chronicle_by_person_query(&database_connection, &Uuid::new_v4()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_chronicle() -> Result<()> {
        let person_id: Uuid = Uuid::from_str(TEST_PERSON_ID)?;
        let postgres_context: PostgresHandler = PostgresHandler::new().await?;
        let database_connection = postgres_context.database_pool.get().await?;
        let test_date: DateTime<Utc> = offset::Utc::now() + Duration::days(7);
        let query_result =
            create_chronicle_query(&database_connection, &test_date, &person_id, &None).await?;
        assert_eq!(test_date, query_result.date_recorded);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_chronicle_with_id() -> Result<()> {
        let chronicle_id: Uuid = Uuid::from_str(TEST_CHRONICLE_ID)?;
        let person_id: Uuid = Uuid::from_str(TEST_PERSON_ID)?;
        let postgres_context: PostgresHandler = PostgresHandler::new().await?;
        let database_connection = postgres_context.database_pool.get().await?;
        let test_date: DateTime<Utc> = offset::Utc::now() + Duration::days(8);
        let query_result: Chronicle = create_chronicle_query(
            &database_connection,
            &test_date,
            &person_id,
            &Some(chronicle_id),
        )
        .await?;
        assert_eq!(test_date, query_result.date_recorded);
        assert_eq!(chronicle_id, query_result.id);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_chronicle_by_id() -> Result<()> {
        let postgres_context: PostgresHandler = PostgresHandler::new().await?;
        let database_connection = postgres_context.database_pool.get().await?;
        let chronicle_id: Uuid = Uuid::from_str(TEST_CHRONICLE_ID)?;
        let query_result: Option<Chronicle> =
            chronicle_by_id_query(&database_connection, &chronicle_id).await?;
        match query_result {
            Some(chronicle) => {
                println!("{}, {}", chronicle.id, chronicle.date_recorded);
            }
            _ => panic!("Did not find chronicle for test date!"),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_get_chronicle_by_date() -> Result<()> {
        let person_id: Uuid = Uuid::from_str(TEST_PERSON_ID)?;
        let postgres_context: PostgresHandler = PostgresHandler::new().await?;
        let database_connection = postgres_context.database_pool.get().await?;
        let test_date: DateTime<Utc> = offset::Utc::now() + Duration::days(7);
        let query_result: Option<Chronicle> =
            chronicle_by_date_query(&database_connection, &test_date, &person_id).await?;
        match query_result {
            Some(chronicle) => {
                println!("{}", chronicle.date_recorded);
            }
            None => panic!("Did not find chronicle for test date!"),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_update_chronicle() -> Result<()> {
        let person_id: Uuid = Uuid::from_str(TEST_PERSON_ID)?;
        let postgres_context: PostgresHandler = PostgresHandler::new().await?;
        let database_connection = postgres_context.database_pool.get().await?;
        let test_date: DateTime<Utc> = offset::Utc::now() + Duration::days(7);
        let query_result: Option<Chronicle> =
            chronicle_by_date_query(&database_connection, &test_date, &person_id).await?;
        if let Some(mut chronicle) = query_result {
            println!("{}", chronicle.date_recorded);
            chronicle.date_recorded = chronicle.date_recorded + Duration::days(1);
            let query_result: Chronicle =
                update_chronicle_query(&database_connection, &chronicle).await?;
            assert_ne!(test_date, query_result.date_recorded);
        } else {
            panic!("Did not find chronicle for test date!")
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_chronicle() -> Result<()> {
        let chronicle_id: Uuid = Uuid::from_str(TEST_CHRONICLE_ID)?;
        let postgres_context: PostgresHandler = PostgresHandler::new().await?;
        let database_connection = postgres_context.database_pool.get().await?;
        delete_chronicle_query(&database_connection, &chronicle_id).await?;
        Ok(())
    }
}
