use anyhow::Result;
use axum::extract::FromRef;
use axum::routing::MethodRouter;
use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
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

#[derive(Clone)]
pub struct ApplicationState {
    postgres_service: PostgresHandler,
    encryption_service: PasswordEncryptionService,
    key: Key,
}

impl FromRef<ApplicationState> for PostgresHandler {
    fn from_ref(state: &ApplicationState) -> Self {
        state.postgres_service.clone()
    }
}

impl FromRef<ApplicationState> for Key {
    fn from_ref(state: &ApplicationState) -> Self {
        state.key.clone()
    }
}

impl FromRef<ApplicationState> for PasswordEncryptionService {
    fn from_ref(state: &ApplicationState) -> Self {
        state.encryption_service.clone()
    }
}

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
    let encryption_service: PasswordEncryptionService = PasswordEncryptionService::new(
        configuration.hash_iterations,
        configuration.site_secret.clone(),
    );

    let application_state: ApplicationState = ApplicationState {
        postgres_service: postgres_service,
        encryption_service: encryption_service,
        key: Key::from(configuration.site_secret.as_bytes()),
    };

    let serve_directory: MethodRouter =
        get_service(ServeDir::new(FRONTEND_DIST_PATH)).handle_error(handle_error);

    info!("Configuring routers...");
    let application_router: Router = Router::new()
        .merge(api::router::authentication::router())
        .merge(api::router::chronicle::router())
        .merge(api::router::person::router())
        .with_state(application_state)
        .fallback_service(serve_directory);

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
