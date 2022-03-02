use anyhow::{anyhow, Context};
use log::info;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use rocket::{
    form::{Form, FromForm},
    get,
    http::{Cookie, CookieJar},
    post,
    response::content::Html,
    routes, State,
};
use sycamore::view;
use tokio::fs;
use tokio_postgres::NoTls;

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
            files::{INDEX_PATH, REGISTRATION_PATH},
            FAILED_LOGIN_MESSAGE, REGISTRATION_SUCCESS_MESSAGE, SUCCESSFUL_LOGIN_MESSAGE,
            SYCAMORE_BODY,
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
fn favicon() -> Option<()> {
    None
}

#[get("/")]
async fn index() -> Result<Html<String>, ApiError> {
    let index_html: String = String::from_utf8(
        fs::read(INDEX_PATH)
            .await
            .context("Something went wrong reading the index file!")?,
    )
    .context("Failed to convert the html to a string.")?;

    let rendered = sycamore::render_to_string(|| {
        view! {
            frontend::App()
        }
    });

    let index_html: String = index_html.replace(SYCAMORE_BODY, &rendered);

    return Ok(Html(index_html));
}

#[get("/registration")]
async fn registration() -> Result<Html<String>, ApiError> {
    let registration_html: String = String::from_utf8(
        fs::read(REGISTRATION_PATH)
            .await
            .map_err(|error| anyhow!("{}", error))?,
    )
    .map_err(|error| anyhow!("{}", error))?;

    return Ok(Html(registration_html));
}

#[post("/register", data = "<registration_form>")]
async fn register(
    postgres_service: &State<PostgresHandler>,
    registration_form: Form<CredentialsForm<'_>>,
) -> Result<String, ApiError> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .map_err(|error| anyhow!("{}", error))?;

    info!("LOREMASTER: Checking for existing users with provided email address...");
    let existing_credentials: Option<Credentials> = credential_by_email_address_query(
        &database_connection,
        &registration_form.email_address.to_string(),
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    if existing_credentials.is_some() {
        info!("LOREMASTER: Existing user found!");
        //TODO: Send an email to the specified address and indicate someone tried to re-register using that email
        return Ok(REGISTRATION_SUCCESS_MESSAGE.to_string());
    }

    info!("LOREMASTER: Email can be registered.");
    let encrypted_password: String =
        PasswordEncryptionService::encrypt_password(&registration_form.password)
            .map_err(|error| anyhow!("{}", error))?;

    info!("LOREMASTER: Adding new user to database...");
    create_person_query(
        &database_connection,
        &registration_form.email_address.to_string(),
        &encrypted_password,
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    return Ok(REGISTRATION_SUCCESS_MESSAGE.to_string());
}

#[post("/authenticate", data = "<authentication_form>")]
async fn authenticate(
    postgres_service: &State<PostgresHandler>,
    cookie_jar: &CookieJar<'_>,
    authentication_form: Form<CredentialsForm<'_>>,
) -> Result<String, ApiError> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .map_err(|error| anyhow!("{}", error))?;

    let query_result: Option<Credentials> = credential_by_email_address_query(
        &database_connection,
        &authentication_form.email_address.to_string(),
    )
    .await
    .map_err(|error| anyhow!("{}", error))?;

    if let Some(person) = query_result {
        let valid_password: bool = PasswordEncryptionService::verify_password(
            &person.encrypted_password,
            &authentication_form.password,
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
        return Ok(SUCCESSFUL_LOGIN_MESSAGE.to_string());
        //return Ok(Redirect::to(uri!(index)));
    } else {
        return Err(ApiError::Anyhow {
            source: anyhow!(FAILED_LOGIN_MESSAGE),
        });
    }
}

#[post("/logout")]
async fn logout(cookie_jar: &CookieJar<'_>) -> Result<String, ApiError> {
    cookie_jar.remove_private(Cookie::named(cookie_fields::USER_ID));
    cookie_jar.remove_private(Cookie::named(cookie_fields::SESSION_ID));
    return Ok("Cookies cleared.".to_string());
}

pub fn routes() -> Vec<rocket::Route> {
    routes![authenticate, favicon, index, logout, registration, register,]
}
