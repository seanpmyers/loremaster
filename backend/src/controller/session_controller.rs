use std::str::FromStr;

use rocket::{
   form::{
      FromForm
   },
   routes, 
   Request, 
   request::{
      self,
      FromRequest
   }
};
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
            .map(|value| -> UserId {UserId(Uuid::from_str(value?)})
            .or_forward(())
    }
}

pub fn routes() -> Vec<rocket::Route> {
   routes![index, no_auth_index, login, login_page, post_login, logout]
}