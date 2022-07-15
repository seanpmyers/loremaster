// use log::error;
// use rocket::{
//     http::Status,
//     response::{self, Responder},
//     Request,
// };
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum ApiError {
//     #[error("Anyhow Error {source:?}")]
//     Anyhow {
//         #[from]
//         source: anyhow::Error,
//     },
// }

// impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
//     fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
//         // log `self` to your favored error tracker, e.g. sentry
//         error!("{}", self);

//         Status::InternalServerError.respond_to(request)
//         // match self {
//         //     // in our simplistic example, we're happy to respond with the default 500 responder in all cases
//         //     _ => Status::InternalServerError.respond_to(request),
//         // }
//     }
// }
