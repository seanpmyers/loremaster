use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Chronicle {
    pub id: Uuid,
    pub person_id: Uuid,
    pub date_recorded: Date,
    pub notes: Option<String>,
    pub creation_time: Option<OffsetDateTime>,
}
