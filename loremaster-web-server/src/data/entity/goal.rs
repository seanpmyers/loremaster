use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::person::PersonId;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: GoalId,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct GoalId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for GoalId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PersonGoal {
    pub person_id: PersonId,
    pub goal_id: GoalId,
    pub goal_name: String,
}
