use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct WebAuthenticationChallenge {
    pub id: Uuid,
    pub user_name: String,
    pub passkey_registration: Value,
}
