use anyhow::{anyhow, Result};
use ctap_hid_fido2::{
    fidokey::{
        get_assertion::get_assertion_params::{Assertion, GetAssertionArgs},
        make_credential::{Attestation, MakeCredentialArgs},
        GetAssertionArgsBuilder, MakeCredentialArgsBuilder,
    },
    verifier, Cfg, FidoKeyHid, FidoKeyHidFactory,
};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//TODO: Where should this live
pub const RELAYING_PARTY_ID: &str = "chronilore.day";
pub const LOCAL_HOST_RELAYING_PARTY_ID: &str = "localhost";
pub const RELAYING_PARTY: &str = "Loremaster";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SecurityKeyChallenge {
    // The id must be a subset of the domain currently in the browser.
    pub relaying_party_id: &'static str,
    // The organization responsible for registering and authenticating the user.
    pub relaying_party: &'static str,
    // The challenge is a buffer of cryptographically random bytes generated on the server, and is needed to prevent "replay attacks"
    pub challenge: [u8; 32],
    pub user_id: Uuid,
}

pub struct PersonSecurityKey {
    pub credential_id: Vec<u8>,
    pub credential_public_key_derivative: Vec<u8>,
}

#[derive(Clone)]
pub struct SecurityKeyService {}

pub trait SecurityKeyAuthentication {
    fn new() -> Self;
    fn create_challenge(&self) -> Result<SecurityKeyChallenge>;
    fn register_key(&self, personal_identification_number: String) -> Result<PersonSecurityKey>;
    fn verify_key(
        &self,
        personal_identification_number: String,
        credential_id: Vec<u8>,
        public_key_derivative: Vec<u8>,
    ) -> bool;
}

impl SecurityKeyService {}

impl SecurityKeyAuthentication for SecurityKeyService {
    fn new() -> Self {
        SecurityKeyService {}
    }

    fn create_challenge(&self) -> Result<SecurityKeyChallenge> {
        Ok(SecurityKeyChallenge {
            relaying_party_id: LOCAL_HOST_RELAYING_PARTY_ID,
            //TODO: fix for prod
            relaying_party: RELAYING_PARTY,
            challenge: verifier::create_challenge(),
            user_id: Uuid::new_v4(),
        })
    }

    fn register_key(&self, personal_identification_number: String) -> Result<PersonSecurityKey> {
        let challenge: [u8; 32] = verifier::create_challenge();

        // create `MakeCredentialArgs`
        let make_credential_args: MakeCredentialArgs =
            MakeCredentialArgsBuilder::new(RELAYING_PARTY_ID, &challenge)
                .pin(&personal_identification_number)
                .build();

        // create `FidoKeyHid`
        let device: FidoKeyHid = FidoKeyHidFactory::create(&Cfg::init()).unwrap();

        // get `Attestation` Object
        let attestation: Attestation = device
            .make_credential_with_args(&make_credential_args)
            .unwrap();
        info!("Register Success");

        // verify `Attestation` Object
        let verify_result: verifier::AttestationVerifyResult =
            verifier::verify_attestation(RELAYING_PARTY_ID, &challenge, &attestation);
        if !verify_result.is_success {
            warn!("Verify Failed");
            return Err(anyhow!(
                "Failed to verify the user security key attestation."
            ));
        }

        Ok(PersonSecurityKey {
            credential_id: verify_result.credential_id,
            credential_public_key_derivative: verify_result.credential_publickey_der,
        })
    }

    fn verify_key(
        &self,
        personal_identification_number: String,
        credential_id: Vec<u8>,
        credential_public_key_derivative: Vec<u8>,
    ) -> bool {
        // create `FidoKeyHid`
        let device: FidoKeyHid = FidoKeyHidFactory::create(&Cfg::init()).unwrap();
        println!("Authenticate");
        // create `challenge`
        let challenge: [u8; 32] = verifier::create_challenge();

        // create `GetAssertionArgs`
        let get_assertion_args: GetAssertionArgs =
            GetAssertionArgsBuilder::new(RELAYING_PARTY_ID, &challenge)
                .pin(&personal_identification_number)
                .credential_id(&credential_id)
                .build();

        // get `Assertion` Object
        let assertions: Vec<Assertion> =
            device.get_assertion_with_args(&get_assertion_args).unwrap();
        println!("- Authenticate Success");

        // verify `Assertion` Object
        if !verifier::verify_assertion(
            RELAYING_PARTY_ID,
            &credential_public_key_derivative,
            &challenge,
            &assertions[0],
        ) {
            println!("- ! Verify Assertion Failed");
        }
        false
    }
}
