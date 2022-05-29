use anyhow::anyhow;
use log::info;
use rocket::{
    form::{Form, FromForm},
    fs::NamedFile,
    get,
    http::{Cookie, CookieJar},
    post,
    response::content::RawHtml,
    routes, State,
};
use tokio::fs;

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
            cookie_fields,
            files::{FAVICON_PATH, INDEX_PATH},
            FAILED_LOGIN_MESSAGE, REGISTRATION_SUCCESS_MESSAGE, SUCCESSFUL_LOGIN_MESSAGE,
        },
        password_encryption::{PasswordEncryption, PasswordEncryptionService},
    },
};

#[derive(FromForm)]
struct CredentialsForm<'r> {
    email_address: &'r str,
    password: &'r str,
}

#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/", $crate::controller:: $($t)*))
}

pub use session_uri as uri;

#[get("/favicon.ico")]
async fn favicon() -> Result<Option<NamedFile>, ApiError> {
    let favicon_file: NamedFile = NamedFile::open(FAVICON_PATH)
        .await
        .map_err(|error| anyhow!("{}", error))?;

    Ok(Some(favicon_file))
}

#[get("/")]
async fn index() -> Result<RawHtml<String>, ApiError> {
    let index_html: String = String::from_utf8(
        fs::read(INDEX_PATH)
            .await
            .map_err(|error| anyhow!("{}", error))?,
    )
    .map_err(|error| anyhow!("{}", error))?;

    Ok(RawHtml(index_html))
}

#[post("/register", data = "<registration_form>")]
async fn register(
    postgres_service: &State<PostgresHandler>,
    registration_form: Form<CredentialsForm<'_>>,
) -> Result<String, ApiError> {
    info!("API CALL: /session/register");
    info!("Checking for existing users with provided email address.");
    let existing_credentials: Option<Credentials> = credential_by_email_address_query(
        &postgres_service.database_pool,
        registration_form.email_address,
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    if existing_credentials.is_some() {
        info!("Existing user found!");
        //TODO: Send an email to the specified address and indicate someone tried to re-register using that email
        return Ok(REGISTRATION_SUCCESS_MESSAGE.to_string());
    }

    info!("Email can be registered.");
    let encrypted_password: String =
        PasswordEncryptionService::encrypt_password(registration_form.password)
            .map_err(|error| anyhow!("{}", error))?;

    info!("Adding new user to database.");
    create_person_query(
        &postgres_service.database_pool,
        registration_form.email_address,
        &encrypted_password,
        None,
        None,
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    Ok(REGISTRATION_SUCCESS_MESSAGE.to_string())
}

#[post("/authenticate", data = "<authentication_form>")]
async fn authenticate(
    postgres_service: &State<PostgresHandler>,
    cookie_jar: &CookieJar<'_>,
    authentication_form: Form<CredentialsForm<'_>>,
) -> Result<String, ApiError> {
    info!("API CALL: /session/authenticate");
    let query_result: Option<Credentials> = credential_by_email_address_query(
        &postgres_service.database_pool,
        authentication_form.email_address,
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    if let Some(person) = query_result {
        let valid_password: bool = PasswordEncryptionService::verify_password(
            &person.encrypted_password,
            authentication_form.password,
        )
        .map_err(|error| anyhow!("{}", error))?;

        if !valid_password {
            return Err(ApiError::Anyhow {
                source: anyhow!(FAILED_LOGIN_MESSAGE),
            });
        }

        cookie_jar.add_private(
            Cookie::build(cookie_fields::USER_ID, person.id.to_string())
                // .http_only(true)
                // .secure(true)
                // .same_site(SameSite::Strict)
                .finish(),
        );
        Ok(SUCCESSFUL_LOGIN_MESSAGE.to_string())
        //return Ok(Redirect::to(uri!(index)));
    } else {
        Err(ApiError::Anyhow {
            source: anyhow!(FAILED_LOGIN_MESSAGE),
        })
    }
}

#[post("/logout")]
async fn logout(cookie_jar: &CookieJar<'_>) -> Result<String, ApiError> {
    info!("API CALL: /session/logout");
    cookie_jar.remove_private(Cookie::named(cookie_fields::USER_ID));
    cookie_jar.remove_private(Cookie::named(cookie_fields::SESSION_ID));
    Ok("Cookies cleared.".to_string())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![authenticate, favicon, index, logout, register,]
}
