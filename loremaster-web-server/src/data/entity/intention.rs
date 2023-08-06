use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Intention {
    pub id: Uuid,
    pub action_id: Uuid,
    pub person_id: Uuid,
    pub chronicle_id: Option<Uuid>,
    pub intended_time: Option<OffsetDateTime>,
    pub expected_duration: Option<Duration>,
}
