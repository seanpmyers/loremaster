use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PersonChroncile {
    pub chronicle_id: Uuid,
    pub chronicle_date: Date,
    pub person_alias: Option<String>,
}
