use std::error::Error;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::error;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("{source:?}")]
    Anyhow {
        #[from]
        source: anyhow::Error,
    },
    //Something went wrong during authentication
    // AuthenticationError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        error!("{self}");

        let body: Json<serde_json::Value> = Json(json!({
            "error": "Something went wrong on our side. Sorry.",
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
