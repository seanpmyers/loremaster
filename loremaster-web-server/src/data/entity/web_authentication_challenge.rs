use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct WebAuthenticationChallenge {
    pub id: WebAuthenticationChallengeId,
    pub user_name: String,
    pub passkey: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct WebAuthenticationChallengeId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for WebAuthenticationChallengeId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

pub type WebAuthenticationRegister = WebAuthenticationChallenge;
pub type WebAuthenticationLogin = WebAuthenticationChallenge;
