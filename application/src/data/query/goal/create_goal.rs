use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};

use uuid::Uuid;

use crate::data::entity::goal::Goal;

const QUERY: &str = "
    INSERT INTO
        public.goal (
            id
            , name
        )
    VALUES 
    ($1, $2)
    RETURNING
        id
        , name
    ;";

pub async fn create_goal_query(database_connection: &PgPool, goal: &String) -> Result<Goal> {
    info!("QUERY CALL: create_action_query");
    let new_id: Uuid = Uuid::new_v4();

    let query_result: Goal = query_as::<_, Goal>(QUERY)
        .bind(&new_id)
        .bind(&goal)
        .fetch_one(database_connection)
        .await?;

    Ok(query_result)
}
