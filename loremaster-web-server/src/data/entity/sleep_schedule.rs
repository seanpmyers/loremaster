use serde::{Deserialize, Serialize};
use time::Time;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SleepSchedule {
    pub id: SleepScheduleId,
    pub start_time: Time,
    pub end_time: Time,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct SleepScheduleId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for SleepScheduleId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
