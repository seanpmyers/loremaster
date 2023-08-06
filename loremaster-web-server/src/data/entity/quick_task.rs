use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct QuickTask {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct PersonQuickTask {
    pub quick_task_id: Uuid,
    pub person_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}
