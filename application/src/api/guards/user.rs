use log::info;
use rocket::{
    outcome::IntoOutcome,
    request::{self, FromRequest},
    Request,
};
use uuid::Uuid;

use crate::utility::constants::cookie_fields::USER_ID;

#[derive(Debug)]
pub struct User(pub Uuid);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        info!("Checking for user_id cookie.");
        let user_id: Option<Uuid> = request
            .cookies()
            .get_private(USER_ID)
            .and_then(|cookie| Uuid::parse_str(cookie.value()).ok());

        return user_id.map(User).or_forward(());
    }
}
