use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Credentials {
    pub email_address: String,
    pub password: String,
}
