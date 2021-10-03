use std::str::FromStr;

use rocket::{Request, form::{
      FromForm
   }, http::CookieJar, outcome::IntoOutcome, request::{
      self,
      FromRequest
   }, routes};
use uuid::Uuid;
use crate::{
   data::entity::person::Credentials, 
};

#[derive(FromForm)]
struct Login<'r> {
    email: &'r str,
    password: &'r str
}

#[derive(Debug)]
struct UserId(Uuid);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserId {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserId, Self::Error> {
        return request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(UserId)
            .or_forward(())
    }
}
