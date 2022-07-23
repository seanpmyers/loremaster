use anyhow::Result;
use axum::{
    http::{
        header::{
            ACCESS_CONTROL_ALLOW_CREDENTIALS, AUTHORIZATION, CONTENT_TYPE, COOKIE, SET_COOKIE,
        },
        HeaderValue, Method,
    },
    Extension, Router,
};
use axum_extra::extract::cookie::Key;
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
    password_encryption::{PasswordEncryption, PasswordEncryptionService},
};

#[tokio::main]
async fn main() -> Result<()> {
    let environment: String = std::env::var(ENVIRONMENT)?;

    configure_logging();
    info!("Starting up!");

    let configuration: LoremasterConfiguration = get_configuration_from_file(&environment)?;

    info!("Attempting a database connection...");
    let postgres_service: PostgresHandler =
        PostgresHandler::new(configuration.postgresql_connection_string).await?;
    info!("Connection established.");

    info!("Creating encryption service...");
    let encryption_service = PasswordEncryptionService::new(
        configuration.hash_iterations,
        configuration.site_secret.clone(),
    );

    let frontend_url: String = format!(
        "{}{}",
        FRONTEND_ORIGIN_URL,
        configuration.frontend_port.to_string()
    );

    info!("Configuring routers...");
    let application_router: Router = Router::new()
        .merge(api::router::session::router())
        .merge(api::router::chronicle::router())
        .layer(Extension(encryption_service))
        .layer(Extension(postgres_service))
        .layer(Extension(Key::from(configuration.site_secret.as_bytes())))
        .layer(
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin(frontend_url.parse::<HeaderValue>()?)
                .allow_headers([
                    AUTHORIZATION,
                    ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    SET_COOKIE,
                    COOKIE,
                    CONTENT_TYPE,
                ])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                    Method::PUT,
                ]),
        );

    let socket_address: SocketAddr =
        SocketAddr::from((configuration.ipv4_address, configuration.port));

    let address_string: String = socket_address.to_string();
    info!("Loremaster servers are available at:\n\n BACKEDND API: > http://{} <\n\n FRONTEND_CLIENT: > {} <\n"
        , address_string
        , frontend_url
    );

    serve(application_router, socket_address).await?;

    info!("Shutting down.");

    Ok(())
}

async fn serve(router: Router, socket_address: SocketAddr) -> Result<()> {
    axum::Server::bind(&socket_address)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

fn configure_logging() {
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
}
