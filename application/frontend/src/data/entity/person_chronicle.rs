use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PersonChroncile {
    pub chronicle_id: String,
    pub chronicle_date: Date,
    pub person_alias: Option<String>,
}
