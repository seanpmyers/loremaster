use chrono::{Date, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Chronicle {
    pub id : Uuid,
    pub date_recorded : Date<Utc>
}