use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct WebAuthenticationChallenge {
    pub id: Uuid,
    pub user_name: String,
    pub passkey: Value,
}

pub type WebAuthenticationRegister = WebAuthenticationChallenge;
pub type WebAuthenticationLogin = WebAuthenticationChallenge;
