use std::str::FromStr;
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
   }, routes
};
use uuid::Uuid;

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

#[get("/registration")]
async fn get_registration() {
    return;
}


// #[post("/login", data = "<login>")]
// fn post_login(jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Result<Redirect, Flash<Redirect>> {
//    jar.add_private(Cookie::new(cookie_fields::USER_ID, 1.to_string()));
//    Ok(Redirect::to(uri!(index)))
//    // Err(Flash::error(Redirect::to(uri!(login_page)), "Unable to verify your identity with the credentials you've provided."))
   
// }

// #[post("/logout")]
// async fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
//     jar.remove_private(Cookie::named("user_id"));
//     Flash::success(Redirect::to(uri!(get_registration)), "Successfully logged out.")
// }


pub fn routes() -> Vec<rocket::Route> {
    routes![
        index
        , get_registration
        ]
 }