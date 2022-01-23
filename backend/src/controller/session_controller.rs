
use anyhow::Context;
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
       Cookie
    }, 
   response::{
      content::Json, 
      Redirect
   }, routes, State
};
use tokio_postgres::NoTls;


use crate::{
    data::{
        postgres_handler::PostgresHandler, 
        query::{
            person::{
                credential_by_email_address::credential_by_email_address_query
            }
        }
    }, 
    utility::password_encryption::{
        PasswordEncryptionService, 
        PasswordEncryption
    }
};

use super::cookie_fields;

#[derive(FromForm)]
struct AuthenticationForm<'r> {
    email_address: &'r str,
    password: &'r str
}

#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/", $crate::controller:: $($t)*))
}

pub use session_uri as uri;

const FAILED_LOGIN_MESSAGE: &str ="Unable to verify your identity with the credentials you've provided.";
const SUCCESSFUL_LOGIN_MESSAGE: &str = "User authenticated successfully!";

#[get("/")]
async fn index() {
   
} 

#[get("/register")]
async fn register() {
    return;
}


#[post("/authenticate", data = "<authentication_form>")]
async fn authenticate(
    postgres_service: &State<PostgresHandler>, 
    cookie_jar: &CookieJar<'_>, 
    authentication_form: Form<AuthenticationForm<'_>>
) -> Json<String> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool
        .get()
        .await
        .context(format!("Failed to get database connection!"))
        .unwrap();

    let query_result = credential_by_email_address_query(
            &database_connection,
            &authentication_form.email_address.to_string()
        )
        .await
        .unwrap();
    
    if let Some(person) = query_result {
        let result = PasswordEncryptionService::verify_password(
            &person.encrypted_password, 
            &authentication_form.password
        ).unwrap();
       
        if result == false { return Json(FAILED_LOGIN_MESSAGE.to_string()); }
    
        cookie_jar.add_private(
            Cookie::new(
                cookie_fields::USER_ID, 
                person.id.to_string()
            )
        );
        return Json(SUCCESSFUL_LOGIN_MESSAGE.to_string());
        //return Ok(Redirect::to(uri!(index)));
       
    }
    else { return Json(FAILED_LOGIN_MESSAGE.to_string()); }
}

#[post("/logout")]
async fn logout(
    cookie_jar: &CookieJar<'_>
) -> Redirect {
    unimplemented!();
    // jar.remove_private(Cookie::named("user_id"));
    // Flash::success(Redirect::to(uri!(get_registration)), "Successfully logged out.")
}


pub fn routes() -> Vec<rocket::Route> {
    routes![
        index
        , register
        , authenticate
        , logout
        ]
 }