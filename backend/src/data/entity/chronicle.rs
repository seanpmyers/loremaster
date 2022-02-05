use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Chronicle {
    pub id: Uuid,
    #[serde(rename(serialize = "dateRecorded"))]
    pub date_recorded: DateTime<Utc>,
}
