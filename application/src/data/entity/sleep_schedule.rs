use serde::{Deserialize, Serialize};
use time::Time;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SleepSchedule {
    pub id: Uuid,
    pub start_time: Time,
    pub end_time: Time,
}
