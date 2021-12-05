// use std::str::FromStr;

// use rocket::{Request, form::{
//       FromForm
//    }, get, http::CookieJar, outcome::IntoOutcome, request::{
//       self,
//       FromRequest
//    }, response::content::Json, routes};
// use uuid::Uuid;
// use crate::{data::entity::person::{Credentials, SessionKey}};

// use super::cookie_fields;

// #[get("/")]
// async fn index(cookies: &CookieJar<'_>) {
//     let session_id = cookies.get_private(cookie_fields::SESSION_ID);
// } 

// #[get("/registration")]
// async fn get_registration() -> () {
//     return ();
// }


// pub fn routes() -> Vec<rocket::Route> {
//     routes![
//         index
//         , get_registration
//         ]
//  }