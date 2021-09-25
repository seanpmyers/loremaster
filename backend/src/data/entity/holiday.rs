use chrono::{Date, Utc};
use uuid::Uuid;

pub struct Holiday {
   pub id: Uuid,
   pub occurence_date: Date<Utc>,
   pub name: String,
}