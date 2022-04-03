use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Chronicle {
    pub id: Uuid,
    #[serde(rename(serialize = "personId"))]
    pub person_id: Uuid,
    #[serde(rename(serialize = "dateRecorded"))]
    pub date_recorded: Date,
    pub notes: Option<String>,
    #[serde(rename(serialize = "creationTime"))]
    pub creation_time: Option<OffsetDateTime>,
}
