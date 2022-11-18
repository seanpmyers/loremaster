use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PersonMeta {
    pub id: String,
    #[serde(rename(deserialize = "emailAddress"))]
    pub email_address: String,
    #[serde(rename(deserialize = "registrationDate"))]
    pub registration_date: OffsetDateTime,
    pub alias: Option<String>,
}
