use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use super::{chronicle::ChronicleId, person::PersonId};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Intention {
    pub id: IntentionId,
    pub action_id: Uuid,
    pub person_id: PersonId,
    pub chronicle_id: Option<ChronicleId>,
    pub intended_time: Option<OffsetDateTime>,
    pub expected_duration: Option<Duration>,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct IntentionId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for IntentionId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
