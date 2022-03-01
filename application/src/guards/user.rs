use log::info;
use rocket::{request::{FromRequest, self}, Request, outcome::IntoOutcome};
use uuid::Uuid;

use crate::controller::cookie_fields::USER_ID;

#[derive(Debug)]
pub struct User (pub Uuid);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        info!("LOREMASTER: Checking for user_id cookie...");
        let user_id: Option<Uuid> = request
            .cookies()
            .get_private(USER_ID)
            .and_then(
                |cookie| Uuid::parse_str(
                    &cookie
                    .value()   
                    .to_owned()
                )
                .ok()
            );

        return user_id
            .map(User)
            .or_forward(());
    }
}