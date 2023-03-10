use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SecurityKeyChallenge {
    // The id must be a subset of the domain currently in the browser.
    pub relaying_party_id: String,
    // The organization responsible for registering and authenticating the user.
    pub relaying_party: String,
    // The challenge is a buffer of cryptographically random bytes generated on the server, and is needed to prevent "replay attacks"
    pub challenge: [u8; 32],
}
