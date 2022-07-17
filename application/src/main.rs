use anyhow::Result;
use axum::{
    http::{HeaderValue, Method},
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use sqlx::types::time::OffsetDateTime;
use std::io::Write;
use std::net::SocketAddr;
use time::format_description::well_known::Rfc3339;
use tower_http::cors::CorsLayer;
use utility::loremaster_configuration::LoremasterConfiguration;

mod api;
mod data;
mod utility;

use data::postgres_handler::PostgresHandler;

use crate::utility::{
    constants::{ENVIRONMENT, FRONTEND_ORIGIN_URL},
    loremaster_configuration::get_configuration_from_file,
};

#[tokio::main]
async fn main() -> Result<()> {
    info!("Starting up.");
    let environment: String = std::env::var(ENVIRONMENT)?;

    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "LOREMASTER_{}: {} [{}] - {}",
                std::env::var(ENVIRONMENT).unwrap(),
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let configuration: LoremasterConfiguration = get_configuration_from_file(environment)?;

    info!("Attempting database connection...");
    let postgres_service: PostgresHandler =
        PostgresHandler::new(configuration.postgresql_connection_string).await?;
    info!("Connection established.");

    let backend_router: Router = Router::new()
        .route("/", get(hello_world))
        .route("/json", get(json))
        .layer(Extension(postgres_service))
        .layer(
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin(FRONTEND_ORIGIN_URL.parse::<HeaderValue>()?)
                .allow_methods([Method::GET]),
        );

    let socket_address: SocketAddr =
        SocketAddr::from((configuration.ipv4_address, configuration.port));
    let address_string: String = socket_address.to_string();
    info!("Launching HTTP server at: http://{address_string}");
    serve(backend_router, socket_address).await?;

    info!("Shutting down.");

    Ok(())
}

async fn serve(router: Router, socket_address: SocketAddr) -> Result<()> {
    axum::Server::bind(&socket_address)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

async fn json() -> impl IntoResponse {
    info!("Json requested!");
    Json(vec!["one", "two", "three"])
}

async fn hello_world() -> Html<&'static str> {
    info!("Hello World requested!");
    Html("<h1>Hello, World!</h1>")
}
