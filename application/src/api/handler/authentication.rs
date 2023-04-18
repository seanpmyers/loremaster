use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use email_address::EmailAddress;
use log::info;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use webauthn_rs::{
    prelude::{
        CreationChallengeResponse, CredentialID, PasskeyRegistration, RegisterPublicKeyCredential,
    },
    Webauthn,
};

use crate::{
    data::{
        entity::{self, person::Credentials},
        query::{
            email_address::create_email_address::create_email_address_query,
            person::{
                create_person::create_person_query,
                credential_by_email_address::credential_by_email_address_query,
            },
        },
    },
    utility::password_encryption::{PasswordEncryption, PasswordEncryptionService},
};

pub enum RegistrationResult {
    Success,
    InvalidEmailAddress,
    BlockedEmailAddress,
    EmailAddressInUse,
    InvalidPassword,
}

const ALLOWED_EMAIL_ADDRESSES: [&str; 2] = ["person@loremaster.xyz", "mail@seanmyers.xyz"];
const MINIMUM_PASSWORD_LENGTH: usize = 8;

pub async fn register_handler(
    database_pool: &Pool<Postgres>,
    encryption_service: &PasswordEncryptionService,
    input_email_address: &str,
    input_password: &str,
) -> Result<RegistrationResult> {
    let clean_email: &str = input_email_address.trim();
    let clean_password: &str = input_password.trim();

    if clean_password.len() < MINIMUM_PASSWORD_LENGTH {
        return Ok(RegistrationResult::InvalidPassword);
    }

    if !EmailAddress::is_valid(clean_email) {
        return Ok(RegistrationResult::InvalidEmailAddress);
    }

    if !ALLOWED_EMAIL_ADDRESSES.contains(&clean_email) {
        return Ok(RegistrationResult::BlockedEmailAddress);
    }

    let valid_email_address: EmailAddress =
        EmailAddress::from_str(clean_email).map_err(|error| anyhow!("{}", error))?;

    info!("Checking for existing users with provided email address.");
    let existing_credentials: Option<Credentials> =
        credential_by_email_address_query(database_pool, &valid_email_address)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    if existing_credentials.is_some() {
        info!("Existing user found!");
        //TODO: Send an email to the specified address and indicate someone tried to re-register using that email
        return Ok(RegistrationResult::EmailAddressInUse);
    }

    info!("Email can be registered.");
    let encrypted_password: String = encryption_service
        .encrypt_password(clean_password)
        .map_err(|error| anyhow!("{}", error))?;

    let new_email_address: entity::email_address::EmailAddress =
        create_email_address_query(database_pool, &valid_email_address).await?;

    info!("Adding new user to database.");
    create_person_query(
        database_pool,
        &new_email_address.id,
        &encrypted_password,
        None,
        None,
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    Ok(RegistrationResult::Success)
}

pub async fn web_authentication_api_register_start_handler(
    web_authentication_service: &Arc<Webauthn>,
    user_name: &str,
    user_display_name: &str,
) -> Result<CreationChallengeResponse> {
    let new_user_id: Uuid = Uuid::new_v4();
    //TODO: exclude any existing credentials
    //TODO: query for existing credentials
    let excluded_credentials: Option<Vec<CredentialID>> = None;
    let (challenge, passkey_registration): (CreationChallengeResponse, PasskeyRegistration) =
        web_authentication_service
            .start_passkey_registration(
                new_user_id,
                user_name,
                user_display_name,
                excluded_credentials,
            )
            .expect("Invalid input during webauthn passkey registration start");
    //TODO: store passkey_registration
    Ok(challenge)
}

pub async fn web_authentication_api_register_finish_handler(
    web_authentication_service: &Arc<Webauthn>,
    user_credential_json: &RegisterPublicKeyCredential,
) -> Result<()> {
    Ok(())
}
