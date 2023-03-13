use std::{net::SocketAddr, str::FromStr};

use anyhow::anyhow;
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    Form, PrivateCookieJar,
};
use email_address::EmailAddress;
use log::{info, warn};
use serde::Deserialize;

use crate::{
    api::{
        handler::authentication::{
            handle_register_with_security_key, register_handler, security_key_challenge_handler,
            RegistrationResult,
        },
        response::ApiError,
    },
    data::{
        entity::person::Credentials, postgres_handler::PostgresHandler,
        query::person::credential_by_email_address::credential_by_email_address_query,
    },
    security::authentication::security_key::{SecurityKeyChallenge, SecurityKeyService},
    utility::{
        constants::{
            cookie_fields, BLOCKED_EMAIL_MESSAGE, FAILED_LOGIN_MESSAGE, INVALID_EMAIL_MESSAGE,
            INVALID_PASSWORD_MESSAGE, REGISTRATION_SUCCESS_MESSAGE, SUCCESSFUL_LOGIN_MESSAGE,
        },
        password_encryption::{PasswordEncryption, PasswordEncryptionService},
    },
    ApplicationState,
};

#[derive(Deserialize, Debug)]
struct CredentialsForm {
    email_address: String,
    password: String,
}

async fn register(
    State(postgres_service): State<PostgresHandler>,
    State(encryption_service): State<PasswordEncryptionService>,
    Form(registration_form): Form<CredentialsForm>,
) -> Result<Response, ApiError> {
    info!("API CALL: /authentication/register");

    match register_handler(
        &postgres_service.database_pool,
        &encryption_service,
        &registration_form.email_address,
        &registration_form.password,
    )
    .await?
    {
        RegistrationResult::Success => Ok((
            StatusCode::ACCEPTED,
            REGISTRATION_SUCCESS_MESSAGE.to_string(),
        )
            .into_response()),
        RegistrationResult::InvalidEmailAddress => {
            Ok((StatusCode::BAD_REQUEST, INVALID_EMAIL_MESSAGE).into_response())
        }
        RegistrationResult::BlockedEmailAddress => {
            Ok((StatusCode::FORBIDDEN, String::from(BLOCKED_EMAIL_MESSAGE)).into_response())
        }
        RegistrationResult::EmailAddressInUse => Ok((
            StatusCode::ACCEPTED,
            REGISTRATION_SUCCESS_MESSAGE.to_string(),
        )
            .into_response()),
        RegistrationResult::InvalidPassword => {
            Ok((StatusCode::BAD_REQUEST, INVALID_PASSWORD_MESSAGE).into_response())
        }
    }
}

async fn authenticate(
    ConnectInfo(socket_address): ConnectInfo<SocketAddr>,
    State(postgres_service): State<PostgresHandler>,
    State(encryption_service): State<PasswordEncryptionService>,
    cookie_jar: PrivateCookieJar,
    Form(authentication_form): Form<CredentialsForm>,
) -> Result<Response, ApiError> {
    info!(
        "API CALL: /authentication/authenticate from {}",
        socket_address.ip().to_string()
    );

    let clean_email: &str = authentication_form.email_address.trim();
    let clean_password: &str = authentication_form.password.trim();

    let valid_email_address: EmailAddress =
        EmailAddress::from_str(&clean_email).map_err(|error| anyhow!("{}", error))?;

    let query_result: Option<Credentials> =
        credential_by_email_address_query(&postgres_service.database_pool, &valid_email_address)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    let Some(person) = query_result else {
        info!("No email found matching user input: {}", clean_email);
        return Err(ApiError::Anyhow {
            source: anyhow!(FAILED_LOGIN_MESSAGE),
        })
    };

    let valid_password: bool = encryption_service
        .verify_password(&person.encrypted_password, clean_password)
        .map_err(|error| anyhow!("{}", error))?;

    if !valid_password {
        warn!("Invalid password for email: {}", clean_email);
        return Err(ApiError::Anyhow {
            source: anyhow!(FAILED_LOGIN_MESSAGE),
        });
    }

    let updated_cookie_jar: PrivateCookieJar = cookie_jar.add(
        Cookie::build(cookie_fields::USER_ID, person.id.to_string())
            .same_site(SameSite::Strict)
            .path("/")
            .http_only(true)
            .secure(true)
            .finish(),
    );

    Ok((updated_cookie_jar, SUCCESSFUL_LOGIN_MESSAGE.to_string()).into_response())
    //return Ok(Redirect::to(uri!(index)));
}

async fn logout(cookie_jar: PrivateCookieJar) -> Result<Response, ApiError> {
    info!("API CALL: /authentication/logout");
    let updated_cookie_jar = cookie_jar
        .remove(Cookie::named(cookie_fields::USER_ID))
        .remove(Cookie::named(cookie_fields::SESSION_ID));
    Ok((updated_cookie_jar, "Successfully logged out.").into_response())
}

async fn security_key_challenge(
    State(security_key_service): State<SecurityKeyService>,
) -> Result<Json<SecurityKeyChallenge>, ApiError> {
    info!("API CALL: /authentication/security-key-challenge");
    let result: SecurityKeyChallenge =
        security_key_challenge_handler(&security_key_service).await?;
    Ok(Json(result))
}

async fn register_with_security_key(
    State(security_key_service): State<SecurityKeyService>,
) -> Result<Response, ApiError> {
    let result = handle_register_with_security_key(&security_key_service, &String::new()).await?;
    Ok((StatusCode::CREATED, "Successfully registered security key").into_response())
}

pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route(
            "/authentication/security-key-challenge",
            get(security_key_challenge),
        )
        .route("/authentication/authenticate", post(authenticate))
        .route("/authentication/logout", post(logout))
        .route("/authentication/register", post(register))
}
