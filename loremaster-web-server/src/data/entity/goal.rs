use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PersonGoal {
    pub person_id: Uuid,
    pub goal_id: Uuid,
    pub goal_name: String,
}
