use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PersonMeta {
    pub id: Uuid,
    #[serde(rename(deserialize = "emailAddress"))]
    pub email_address: String,
    #[serde(rename(deserialize = "registrationDate"))]
    pub registration_date: OffsetDateTime,
    pub alias: Option<String>,
}
