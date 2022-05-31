use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Chronicle {
    pub id: String,
    #[serde(rename(serialize = "personId"))]
    pub person_id: String,
    #[serde(rename(serialize = "dateRecorded"))]
    pub date_recorded: Date,
    pub notes: Option<String>,
    // #[serde(rename(serialize = "creationTime"))]
    pub creation_time: Option<OffsetDateTime>,
}
