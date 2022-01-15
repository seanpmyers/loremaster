use std::str::FromStr;
use anyhow::Context;
use log::info;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use rocket::{Request, form::{
      FromForm,
      Form
   }, 
   get,
   post, 
   http::{CookieJar, Cookie}, 
   outcome::IntoOutcome, 
   request::{
      self,
      FromRequest
   }, 
   response::{
      content::Json, 
      Redirect, Flash
   }, routes, State
};
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::{
    data::{
        postgres_handler::PostgresHandler, 
        query
    }, 
    utility::password_encryption::{
        PasswordEncryptionService, 
        PasswordEncryption
    }
};

use super::cookie_fields;

#[derive(FromForm)]
struct Login<'r> {
    username: &'r str,
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

#[get("/register")]
async fn register() {
    return;
}


#[post("/login", data = "<login>")]
async fn login(
    postgres_service: &State<PostgresHandler>, 
    cookie_jar: &CookieJar<'_>, 
    login: Form<Login<'_>>
) -> Result<Redirect, Flash<Redirect>> {
    unimplemented!();
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool
        .get()
        .await
        .context(format!("Failed to get database connection!"))
        .unwrap();

    let query_result = query::person_entity::credentials::by_email_address(
            &database_connection,
            &"".to_string()
        )
        .await
        .unwrap();
    
    let result = PasswordEncryptionService::verify_password(
        &query_result.encrypted_password, 
        &login.password
    ).unwrap();
   
    if result == false {  
        // Err(Flash::error(Redirect::to(
        //     uri!(login)
        // ), 
        // "Unable to verify your identity with the credentials you've provided.")
        // );  
    }

    cookie_jar.add_private(
        Cookie::new(
            cookie_fields::USER_ID, 
            query_result.id.to_string()
        )
    );
//    Ok(Redirect::to(uri!(index)))
   //
   
}

#[post("/logout")]
async fn logout(
    cookie_jar: &CookieJar<'_>
) -> Flash<Redirect> {
    unimplemented!();
    // jar.remove_private(Cookie::named("user_id"));
    // Flash::success(Redirect::to(uri!(get_registration)), "Successfully logged out.")
}


pub fn routes() -> Vec<rocket::Route> {
    routes![
        index
        , register
        , login
        , logout
        ]
 }