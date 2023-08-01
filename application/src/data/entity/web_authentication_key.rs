use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct WebAuthenticationKey {
    pub id: Uuid,
    pub credential_id: Vec<u8>,
    pub cose_algorithm: i32,
    pub passkey: Value,
}
