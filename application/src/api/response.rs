use axum::response::{IntoResponse, Response};
use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Anyhow Error {source:?}")]
    Anyhow {
        #[from]
        source: anyhow::Error,
    },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // log `self` to your favored error tracker, e.g. sentry
        error!("{}", self);

        Status::InternalServerError.respond_to(request)
        // match self {
        //     // in our simplistic example, we're happy to respond with the default 500 responder in all cases
        //     _ => Status::InternalServerError.respond_to(request),
        // }
    }
}
