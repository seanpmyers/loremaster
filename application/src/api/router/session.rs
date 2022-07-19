use anyhow::anyhow;
use axum::{response::IntoResponse, routing::post, Extension, Router};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    Form, PrivateCookieJar,
};
use log::info;
use serde::Deserialize;

use crate::{
    api::response::ApiError,
    data::{
        entity::person::Credentials,
        postgres_handler::PostgresHandler,
        query::person::{
            create_person::create_person_query,
            credential_by_email_address::credential_by_email_address_query,
        },
    },
    utility::{
        constants::{
            cookie_fields, FAILED_LOGIN_MESSAGE, REGISTRATION_SUCCESS_MESSAGE,
            SUCCESSFUL_LOGIN_MESSAGE,
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
    postgres_service: Extension<PostgresHandler>,
    encryption_service: Extension<PasswordEncryptionService>,
    Form(registration_form): Form<CredentialsForm>,
) -> Result<String, ApiError> {
    info!("API CALL: /session/register");
    info!("Checking for existing users with provided email address.");
    let existing_credentials: Option<Credentials> = credential_by_email_address_query(
        &postgres_service.database_pool,
        &registration_form.email_address,
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    if existing_credentials.is_some() {
        info!("Existing user found!");
        //TODO: Send an email to the specified address and indicate someone tried to re-register using that email
        return Ok(REGISTRATION_SUCCESS_MESSAGE.to_string());
    }

    info!("Email can be registered.");
    let encrypted_password: String = encryption_service
        .encrypt_password(&registration_form.password)
        .map_err(|error| anyhow!("{}", error))?;

    info!("Adding new user to database.");
    create_person_query(
        &postgres_service.database_pool,
        &registration_form.email_address,
        &encrypted_password,
        None,
        None,
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    Ok(REGISTRATION_SUCCESS_MESSAGE.to_string())
}

async fn authenticate(
    postgres_service: Extension<PostgresHandler>,
    encryption_service: Extension<PasswordEncryptionService>,
    cookie_jar: PrivateCookieJar,
    Form(authentication_form): Form<CredentialsForm>,
) -> Result<impl IntoResponse, ApiError> {
    info!("API CALL: /session/authenticate");
    let query_result: Option<Credentials> = credential_by_email_address_query(
        &postgres_service.database_pool,
        &authentication_form.email_address,
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    if let Some(person) = query_result {
        let valid_password: bool = encryption_service
            .verify_password(&person.encrypted_password, &authentication_form.password)
            .map_err(|error| anyhow!("{}", error))?;

        if !valid_password {
            return Err(ApiError::Anyhow {
                source: anyhow!(FAILED_LOGIN_MESSAGE),
            });
        }

        let updated_cookie_jar: PrivateCookieJar = cookie_jar.add(
            Cookie::build(cookie_fields::USER_ID, person.id.to_string())
                .same_site(SameSite::Strict)
                .http_only(true)
                .secure(true)
                .finish(),
        );

        Ok((updated_cookie_jar, SUCCESSFUL_LOGIN_MESSAGE.to_string()))
        //return Ok(Redirect::to(uri!(index)));
    } else {
        Err(ApiError::Anyhow {
            source: anyhow!(FAILED_LOGIN_MESSAGE),
        })
    }
}

async fn logout(cookie_jar: PrivateCookieJar) -> Result<impl IntoResponse, ApiError> {
    info!("API CALL: /session/logout");
    let updated_cookie_jar = cookie_jar
        .remove(Cookie::named(cookie_fields::USER_ID))
        .remove(Cookie::named(cookie_fields::SESSION_ID));
    Ok((updated_cookie_jar, "Successfully logged out."))
}

pub fn router() -> Router {
    Router::new()
        .route("/authenticate", post(authenticate))
        .route("/logout", post(logout))
        .route("/register", post(register))
}
