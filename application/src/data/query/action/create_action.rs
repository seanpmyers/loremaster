use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};

use uuid::Uuid;

use crate::data::entity::action::Action;

const QUERY: &str = "
    INSERT INTO
        public.action (
            id
            , name
        )
    VALUES 
    ($1, $2)
    RETURNING
        id
        , name
    ;";

pub async fn create_action_query(database_connection: &PgPool, action: &String) -> Result<Action> {
    info!("QUERY CALL: create_action_query");
    let new_id: Uuid = Uuid::new_v4();

    let query_result: Action = query_as::<_, Action>(QUERY)
        .bind(&new_id)
        .bind(&action)
        .fetch_one(database_connection)
        .await?;

    Ok(query_result)
}
