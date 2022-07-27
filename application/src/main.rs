use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Extension, Router};
use axum_extra::extract::cookie::Key;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use sqlx::types::time::OffsetDateTime;
use std::io::{self, Write};
use std::net::SocketAddr;
use time::format_description::well_known::Rfc3339;
use tower_http::services::ServeDir;
use utility::loremaster_configuration::LoremasterConfiguration;

mod api;
mod data;
mod utility;

use data::postgres_handler::PostgresHandler;

use crate::utility::{
    constants::{files::FRONTEND_DIST_PATH, ENVIRONMENT},
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

    info!("Configuring routers...");
    let application_router: Router = Router::new()
        .merge(api::router::authentication::router())
        .merge(api::router::chronicle::router())
        .layer(Extension(encryption_service))
        .layer(Extension(postgres_service))
        .layer(Extension(Key::from(configuration.site_secret.as_bytes())))
        .fallback(get_service(ServeDir::new(FRONTEND_DIST_PATH)).handle_error(handle_error));

    let socket_address: SocketAddr =
        SocketAddr::from((configuration.ipv4_address, configuration.port));

    let address_string: String = socket_address.to_string();
    info!(
        "Loremaster servers are available at:\n\n BACKEND API: > http://{} <\n",
        address_string
    );

    serve(application_router, socket_address).await?;

    info!("Shutting down.");

    Ok(())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
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
