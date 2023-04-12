use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::{cookie::Key, PrivateCookieJar};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utility::constants::cookie_fields::USER_ID;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct User(pub Uuid);
// TODO: check database for user id/session id
#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
    Key: FromRef<S>,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_result = PrivateCookieJar::<Key>::from_request_parts(parts, state).await;
        match cookie_result {
            Ok(cookie_jar) => match cookie_jar.get(USER_ID) {
                Some(cookie) => {
                    let user_id = Uuid::parse_str(cookie.value()).unwrap();
                    Ok(User(user_id))
                }
                None => Err((StatusCode::UNAUTHORIZED, "No authorization found!")),
            },
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to evaluate authorization.",
            )),
        }
    }
}
