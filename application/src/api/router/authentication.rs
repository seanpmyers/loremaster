use std::{net::SocketAddr, str::FromStr, sync::Arc};

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
use serde::{Deserialize, Serialize};
use webauthn_rs::{prelude::RegisterPublicKeyCredential, Webauthn};

use crate::{
    api::{
        handler::authentication::{
            register_handler, web_authentication_api_register_finish_handler,
            web_authentication_api_register_start_handler, RegistrationResult,
        },
        response::ApiError,
        web_server::ApplicationState,
    },
    data::{
        entity::person::Credentials, postgres_handler::PostgresHandler,
        query::person::credential_by_email_address::credential_by_email_address_query,
    },
    utility::{
        constants::{
            cookie_fields, BLOCKED_EMAIL_MESSAGE, FAILED_LOGIN_MESSAGE, INVALID_EMAIL_MESSAGE,
            INVALID_PASSWORD_MESSAGE, REGISTRATION_SUCCESS_MESSAGE, SUCCESSFUL_LOGIN_MESSAGE,
        },
        password_encryption::{PasswordEncryption, PasswordEncryptionService},
    },
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
        RegistrationResult::Valid => Ok((
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
        EmailAddress::from_str(clean_email).map_err(|error| anyhow!("{}", error))?;

    let query_result: Option<Credentials> =
        credential_by_email_address_query(&postgres_service.database_pool, &valid_email_address)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    let Some(person) = query_result else {
        info!("No email found matching user input: {}", clean_email);
        return Ok((StatusCode::BAD_REQUEST, FAILED_LOGIN_MESSAGE).into_response());
    };

    let valid_password: bool = encryption_service
        .verify_password(&person.encrypted_password, clean_password)
        .map_err(|error| anyhow!("{}", error))?;

    if !valid_password {
        warn!("Invalid password for email: {}", clean_email);
        return Ok((StatusCode::BAD_REQUEST, FAILED_LOGIN_MESSAGE).into_response());
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

#[derive(Deserialize, Debug)]
struct WebAuthenticationRegistrationForm {
    email_address: String,
    alias: String,
}

async fn web_authentication_api_register_start(
    State(postgres_service): State<PostgresHandler>,
    State(web_authentication_service): State<Arc<Webauthn>>,
    Form(web_authentication_registration_form): Form<WebAuthenticationRegistrationForm>,
) -> Result<Response, ApiError> {
    info!("API CALL: /authentication/webauthn/start");
    match web_authentication_api_register_start_handler(
        &postgres_service.database_pool,
        &web_authentication_service,
        &web_authentication_registration_form.email_address,
        &web_authentication_registration_form.alias,
    )
    .await?
    {
        (RegistrationResult::Valid, Some(credential_challenge)) => {
            Ok((StatusCode::ACCEPTED, Json(Some(credential_challenge))).into_response())
        }
        (RegistrationResult::InvalidEmailAddress, _) => {
            Ok((StatusCode::BAD_REQUEST, INVALID_EMAIL_MESSAGE).into_response())
        }
        (RegistrationResult::BlockedEmailAddress, _) => {
            Ok((StatusCode::FORBIDDEN, String::from(BLOCKED_EMAIL_MESSAGE)).into_response())
        }
        (RegistrationResult::EmailAddressInUse, _) => Ok((
            StatusCode::ACCEPTED,
            REGISTRATION_SUCCESS_MESSAGE.to_string(),
        )
            .into_response()),
        //TODO: Log it
        (_, _) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong on our side.",
        )
            .into_response()),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RegistrationInput {
    pub email_address: String,
    pub user_credential_json: RegisterPublicKeyCredential,
}

async fn web_authentication_api_register_finish(
    State(postgres_service): State<PostgresHandler>,
    State(web_authentication_service): State<Arc<Webauthn>>,
    Json(registration_input): Json<RegistrationInput>,
) -> Result<Response, ApiError> {
    info!("API CALL: /authentication/webauthn/finish");
    web_authentication_api_register_finish_handler(
        &postgres_service.database_pool,
        &web_authentication_service,
        &registration_input.email_address,
        &registration_input.user_credential_json,
    )
    .await?;
    Ok((StatusCode::CREATED, "Successfully registered security key").into_response())
}

pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route(
            "/authentication/webauthn/start",
            post(web_authentication_api_register_start),
        )
        .route("/authentication/authenticate", post(authenticate))
        .route("/authentication/logout", post(logout))
        .route("/authentication/register", post(register))
        .route(
            "/authentication/webauthn/finish",
            post(web_authentication_api_register_finish),
        )
}
