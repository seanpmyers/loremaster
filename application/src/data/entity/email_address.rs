use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EmailAddress {
    pub id: Uuid,
    pub display: String,
    pub local_part: String,
    pub domain: String,
    pub validated: bool,
    pub validation_date: Option<OffsetDateTime>,
    pub creation_date: OffsetDateTime,
}
