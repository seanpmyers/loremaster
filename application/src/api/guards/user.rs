use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    http::StatusCode,
};
use axum_extra::extract::cookie::Cookie;
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utility::constants::cookie_fields::{self, USER_ID};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct UserId(Uuid);

pub enum User {
    Found(UserId),
    None,
}

#[async_trait]
impl<B> FromRequest<B> for User {
    type Rejection = (StatusCode, &'static str);

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = Option::<TypedHeader<Cookie>>::from_request(request)
            .await
            .unwrap();

        let session_cookie = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(cookie_fields::USER_ID));

        match session_cookie {
            _ => Ok(Self::Found(UserId(Uuid::new_v4()))),
            None => Err((StatusCode::BAD_REQUEST, "No session cookie found!")),
        }
    }

    // type Error = std::convert::Infallible;

    // async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
    //     info!("Checking for user_id cookie.");
    //     let user_id: Option<Uuid> = request
    //         .cookies()
    //         .get_private(USER_ID)
    //         .and_then(|cookie| Uuid::parse_str(cookie.value()).ok());

    //     return user_id.map(User).or_forward(());
    // }
}
