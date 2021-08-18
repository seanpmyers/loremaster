use chrono::{Date, Local};
use uuid::Uuid;

#[derive(Debug)]
pub struct Chronicle {
    pub id : Uuid,
    pub date_recorded : Date<Local>
}