use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::Cookie,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utility::constants::cookie_fields;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct User(pub Uuid);

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let session_cookie: Option<&str> = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(cookie_fields::USER_ID));
        match session_cookie {
            Some(value) => Ok(User(Uuid::parse_str(value).unwrap())),
            None => Err((StatusCode::UNAUTHORIZED, "No `user_id` found!")),
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
