use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use axum_extra::extract::{cookie::Key, PrivateCookieJar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utility::constants::cookie_fields::USER_ID;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct User(pub Uuid);

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie_result = PrivateCookieJar::<Key>::from_request(req).await;
        match cookie_result {
            Ok(cookie_jar) => match cookie_jar.get(USER_ID) {
                Some(cookie) => {
                    let user_id = Uuid::parse_str(cookie.value()).unwrap();
                    Ok(User(user_id))
                }
                None => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to match authorization.",
                )),
            },
            Err(_) => Err((StatusCode::UNAUTHORIZED, "No authorization found!")),
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
