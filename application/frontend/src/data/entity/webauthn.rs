use serde::{Deserialize, Serialize};
use webauthn_rs_proto::{PublicKeyCredential, RegisterPublicKeyCredential};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebAuthenticationInput {
    pub email_address: String,
    pub user_credential_json: RegisterPublicKeyCredential,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonPublicKeyCredential {
    pub email_address: String,
    pub public_key_credential: PublicKeyCredential,
}
