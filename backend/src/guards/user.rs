use rocket::{request::{FromRequest, self}, Request, outcome::IntoOutcome};
use uuid::Uuid;

#[derive(Debug)]
pub struct User (pub Uuid);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        
        let user_id = request.cookies()
            .get_private("user_id")
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
