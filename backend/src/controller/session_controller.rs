use anyhow::{Context, Error};
use log::info;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use rocket::{
    form::{
      FromForm,
      Form
   }, 
   get,
   post, 
   http::{
       CookieJar, 
       Cookie, 
       SameSite
    }, 
   response::{
      content::Json
   }, 
   routes, 
   State
};
use tokio_postgres::NoTls;


use crate::{
    data::{
        postgres_handler::PostgresHandler, 
        query::{
            person::{
                credential_by_email_address::credential_by_email_address_query, 
                create_person::create_person_query
            }
        }, 
        entity::person::{
            Credentials, 
            Person
        }
    }, 
    utility::{
        password_encryption::{
            PasswordEncryptionService, 
            PasswordEncryption
        },
        constants::{
            FAILED_LOGIN_MESSAGE,
            SUCCESSFUL_LOGIN_MESSAGE, 
            REGISTRATION_SUCCESS_MESSAGE, 
            REGISTRATION_FAILURE_MESSAGE
        }   
    }
};

use super::cookie_fields::{self, USER_ID, SESSION_ID};

#[derive(FromForm)]
struct CredentialsForm<'r> {
    email_address: &'r str,
    password: &'r str
}

#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/", $crate::controller:: $($t)*))
}

pub use session_uri as uri;

#[get("/")]
async fn index() {
   
} 


#[post("/register", data = "<registration_form>")]
async fn register(
    postgres_service: &State<PostgresHandler>, 
    registration_form: Form<CredentialsForm<'_>>
) -> Json<String> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool
        .get()
        .await
        .context("Failed to get database connection!".to_string())
        .unwrap();
    info!("LOREMASTER: Checking for existing users with provided email address...");
    let existing_credentials: Option<Credentials> = credential_by_email_address_query(
        &database_connection, 
        &registration_form.email_address.to_string()
    ).await.unwrap();
    
    if existing_credentials.is_some() {
        info!("LOREMASTER: Existing user found!");
        //Send an email to the specified address and indicate someone tried to re-register using that email
        return Json(REGISTRATION_SUCCESS_MESSAGE.to_string());
    }
    info!("LOREMASTER: Email can be registered.");
    let encrypted_password = PasswordEncryptionService::encrypt_password(
        &registration_form.password
    ).unwrap();
    info!("LOREMASTER: Adding new user to database...");
    let query_result: Result<Person, Error> = create_person_query(
        &database_connection, 
        &registration_form.email_address.to_string(), 
        &encrypted_password
    ).await;

    match query_result {
    Ok(_) => return Json(REGISTRATION_SUCCESS_MESSAGE.to_string()),
    Err(_) => return Json(REGISTRATION_FAILURE_MESSAGE.to_string()),
}

    
    
}


#[post("/authenticate", data = "<authentication_form>")]
async fn authenticate(
    postgres_service: &State<PostgresHandler>, 
    cookie_jar: &CookieJar<'_>, 
    authentication_form: Form<CredentialsForm<'_>>
) -> Json<String> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool
        .get()
        .await
        .context("Failed to get database connection!".to_string())
        .unwrap();

    let query_result: Option<Credentials> = credential_by_email_address_query(
            &database_connection,
            &authentication_form.email_address.to_string()
        )
        .await
        .unwrap();
    
    if let Some(person) = query_result {
        let valid_password: bool = PasswordEncryptionService::verify_password(
            &person.encrypted_password, 
            &authentication_form.password
        ).unwrap();
       
        if !valid_password { return Json(FAILED_LOGIN_MESSAGE.to_string()); }
    
        cookie_jar.add_private(
            Cookie::build(
                cookie_fields::USER_ID, 
                person.id.to_string()
            )
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .finish()
        );
        return Json(SUCCESSFUL_LOGIN_MESSAGE.to_string());
        //return Ok(Redirect::to(uri!(index)));
       
    }
    else { return Json(FAILED_LOGIN_MESSAGE.to_string()); }
}

#[post("/logout")]
async fn logout(
    cookie_jar: &CookieJar<'_>
) -> Json<String> {
    cookie_jar
        .remove_private(Cookie::named(USER_ID));
    cookie_jar
        .remove_private(Cookie::named(SESSION_ID));
    return Json("Cookies cleared.".to_string());
}


pub fn routes() -> Vec<rocket::Route> {
    routes![
        index
        , register
        , authenticate
        , logout
        ]
 }