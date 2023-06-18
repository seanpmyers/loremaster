use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use email_address::EmailAddress;
use log::info;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use webauthn_rs::{
    prelude::{
        CreationChallengeResponse, CredentialID, Passkey, PasskeyRegistration, PublicKeyCredential,
        RegisterPublicKeyCredential,
    },
    Webauthn,
};

use crate::{
    data::{
        entity::{
            self,
            person::Credentials,
            web_authentication_challenge::WebAuthenticationChallenge, web_authentication_key::WebAuthenticationKey,
        },
        query::{
            authentication::{
                add_web_authentication_challenge::add_web_authentication_challenge_query,
                get_web_authentication_by_user_name::get_web_authentication_by_user_name_query,
                get_web_authentication_challenges::get_optional_web_authentication_id_by_user_name_query,
                remove_stale_web_authentication_challenges_by_user_name::remove_stale_web_authentication_challenges_by_user_name_query, self,
            },
            email_address::{
                create_email_address::create_email_address_query,
                email_address_in_use::email_address_in_use_query,
            },
            person::{
                create_person::create_person_query,
                credential_by_email_address::credential_by_email_address_query, self,
            }, password
        },
    },
    utility::password_encryption::{PasswordEncryption, PasswordEncryptionService},
};

pub enum RegistrationResult {
    Valid,
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
    //TODO: ensure sanitized/clean
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
            .await?;

    if existing_credentials.is_some() {
        info!("Existing user found!");
        //TODO: Send an email to the specified address and indicate someone tried to re-register using that email
        return Ok(RegistrationResult::EmailAddressInUse);
    }
    info!("Email can be registered.");

    let new_email_address: entity::email_address::EmailAddress =
        create_email_address_query(database_pool, &valid_email_address).await?;

    info!("Adding new user to database.");
    let person: entity::person::Person = create_person_query(
        database_pool,
        &new_email_address.id,
        None,
        None,
    )
    .await?;

    let encrypted_password: String = encryption_service
    .encrypt_password(clean_password)?;

    let new_password_id: Uuid = password::add_password::add_password_query(database_pool, &encrypted_password).await?;

    //TODO:If anything fails up to here we need to handle the person/password not in use
    // possibly rewrite query to be one transaction instead of individual queries
    person::add_password::add_password_query(database_pool,&person.id, &new_password_id).await?;

    Ok(RegistrationResult::Valid)
}

pub async fn web_authentication_api_register_start_handler(
    database_pool: &Pool<Postgres>,
    web_authentication_service: &Arc<Webauthn>,
    email_address: &str,
    user_alias: &str,
) -> Result<(RegistrationResult, Option<CreationChallengeResponse>)> {
    //TODO: ensure sanitized/clean
    let clean_email: &str = email_address.trim();
    info!("{}", clean_email);
    if !EmailAddress::is_valid(clean_email) {
        return Ok((RegistrationResult::InvalidEmailAddress, None));
    }

    if !ALLOWED_EMAIL_ADDRESSES.contains(&clean_email) {
        return Ok((RegistrationResult::BlockedEmailAddress, None));
    }

    let valid_email_address: EmailAddress =
        EmailAddress::from_str(clean_email).map_err(|error| anyhow!("{}", error))?;

    info!("Checking if the provided email address is already in use.");
    let email_in_use: bool = email_address_in_use_query(database_pool, &valid_email_address)
        .await
        .map_err(|error| anyhow!("{}", error))?;

    if email_in_use {
        info!("Existing user found!");
        //TODO: Send an email to the specified address and indicate someone tried to re-register using that email
        return Ok((RegistrationResult::EmailAddressInUse, None));
    }
    info!("Email can be registered.");

    let user_id: Uuid = get_optional_web_authentication_id_by_user_name_query(
        database_pool,
        valid_email_address.as_str(),
    )
    .await?
    .unwrap_or_else(Uuid::new_v4);

    remove_stale_web_authentication_challenges_by_user_name_query(
        database_pool,
        valid_email_address.as_str(),
    )
    .await?;
    //TODO: exclude any existing credentials
    //TODO: query for existing credentials

    let excluded_credentials: Option<Vec<CredentialID>> = None;
    let (challenge, passkey_registration): (CreationChallengeResponse, PasskeyRegistration) =
        web_authentication_service
            .start_passkey_registration(
                user_id,
                valid_email_address.as_str(),
                user_alias,
                excluded_credentials,
            )
            .expect("Invalid input during webauthn passkey registration start");

    add_web_authentication_challenge_query(
        database_pool,
        &WebAuthenticationChallenge {
            id: user_id,
            user_name: valid_email_address.as_str().to_string(),
            passkey_registration: serde_json::to_value(passkey_registration)?,
        },
    )
    .await?;

    Ok((RegistrationResult::Valid, Some(challenge)))
}

pub async fn web_authentication_api_register_finish_handler(
    database_pool: &Pool<Postgres>,
    web_authentication_service: &Arc<Webauthn>,
    email_address: &str,
    user_credential: &RegisterPublicKeyCredential,
) -> Result<RegistrationResult> {
    let clean_email: &str = email_address.trim();

    if !EmailAddress::is_valid(email_address) {
        return Ok(RegistrationResult::InvalidEmailAddress);
    }

    if !ALLOWED_EMAIL_ADDRESSES.contains(&email_address) {
        return Ok(RegistrationResult::InvalidEmailAddress);
    }

    let valid_email_address: EmailAddress =
        EmailAddress::from_str(clean_email).map_err(|error| anyhow!("{}", error))?;

    let Some(challenge) = get_web_authentication_by_user_name_query(database_pool, email_address).await? 
    else { return Ok(RegistrationResult::InvalidEmailAddress); };

    let state: PasskeyRegistration = serde_json::from_value(challenge.passkey_registration)?;

    let passkey: Passkey = web_authentication_service
        .finish_passkey_registration(user_credential, &state)
        .expect("Invalid input during webauthn passkey registration finish");


    let new_email_address = create_email_address_query(database_pool, &valid_email_address).await?;

    let key: WebAuthenticationKey = WebAuthenticationKey {
        id: Uuid::new_v4(),
        credential_id: passkey.cred_id().0.clone(),
        cose_algorithm: *passkey.cred_algorithm() as i32,
        passkey: serde_json::to_value(passkey)?
    };

    authentication::add_web_authentication_key::add_web_authentication_key_query(database_pool, &key).await?;

    info!("Adding new user to database.");
    let person: entity::person::Person = create_person_query(
        database_pool,
        &new_email_address.id,
        None,
        None,
    )
    .await?;

    //TODO: create relation between person and key
    person::add_web_authentication_key::add_web_authentication_key_query(database_pool, &person.id,&key.id).await?;

    Ok(RegistrationResult::Valid)
}

pub async fn web_authentication_api_login_start_handler(
    database_pool: &Pool<Postgres>,
    web_authentication_service: &Arc<Webauthn>,
    email_address: &str,
) -> Result<()> {
    // let passkeys: Vec<Passkey> = get_passkeys_by_email_address_query().await?;
    // let thing = web_authentication_service.start_passkey_authentication(&passkeys);
    Ok(())
}

pub async fn web_authentication_api_login_finish_handler(
    database_pool: &Pool<Postgres>,
    web_authentication_service: &Arc<Webauthn>,
    public_key_credential: &PublicKeyCredential,
) -> Result<()> {
    // let thing =
    //     web_authentication_service.finish_passkey_authentication(public_key_credential, state);
    Ok(())
}
