// use anyhow::Context;
// use chrono::{offset, DateTime, Utc};
// use log::info;
// use mobc::Connection;
// use mobc_postgres::PgConnectionManager;
// use rocket::{
//     State,
//     // get,
//     // routes,
//     post,
//     // delete,
// //     http::{
// //       Cookie,
// //       CookieJar
// //    },
// };
// use tokio_postgres::{NoTls};

// use crate::data::{
//     query::person::create_person::create_person_query,
//     postgres_handler::PostgresHandler
// };

// // /// Retrieve the user's ID, if any.
// // #[get("/user_id")]
// // fn user_id(cookies: &CookieJar<'_>) -> Option<String> {
// //     cookies.get_private("user_id")
// //         .map(|crumb| format!("User ID: {}", crumb.value()))
// // }

// // /// Remove the `user_id` cookie.
// // #[post("/logout")]
// // fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
// //     cookies.remove_private(Cookie::named("user_id"));
// //     Flash::success(Redirect::to("/"), "Successfully logged out.")
// // }

// // pub fn routes() -> Vec<rocket::Route> {
// //     routes![]
// //  }

// // #[post("/create")]
// // pub async fn create_person (postgres_service: &State<PostgresHandler>) -> Option<String> {
// //     info!("Connecting to database.");
// //     let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
// //         .database_pool
// //         .get()
// //         .await
// //         .context("Failed to get database connection!".to_string())
// //         .unwrap();

// //     let email_address = "email";
// //     let encrypted_password = "password";
// //     let creation_date: DateTime<Utc> = offset::Utc::now();
// //     let _person = create_person_query(
// //         &database_connection,
// //         &email_address.to_string(),
// //         &encrypted_password.to_string(),
// //         ).await;
// //     unimplemented!();
// //     // return None;
// // }
