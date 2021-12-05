use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Chronicle {
    pub id : Uuid,
    pub date_recorded : DateTime<Utc>
}