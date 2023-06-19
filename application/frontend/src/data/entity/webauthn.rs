use serde::{Deserialize, Serialize};
use webauthn_rs_proto::RegisterPublicKeyCredential;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegistrationInput {
    pub email_address: String,
    pub user_credential_json: RegisterPublicKeyCredential,
}
