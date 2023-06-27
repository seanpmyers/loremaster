use crate::api;
use crate::configuration::application::LoremasterWebServerConfiguration;
use crate::utility::constants::files::{
    FRONTEND_DIST_PATH, SSL_CERT_FILE_PATH, SSL_CERT_KEY_FILE_PATH, SSL_CERT_PATH,
};
use crate::utility::constants::{
    DEV_RELAYING_PARTY_ID, LOCAL_HOST_RELAYING_PARTY_ID, QA_RELAYING_PARTY_ID, RELAYING_PARTY,
    RELAYING_PARTY_ID,
};

use crate::utility::password_encryption::PasswordEncryption;
use crate::{
    data::postgres_handler::PostgresHandler,
    utility::password_encryption::PasswordEncryptionService,
};
use anyhow::Result;
use axum::extract::FromRef;
use axum::routing::MethodRouter;
use axum::{routing::get_service, Router};
use axum_extra::extract::cookie::Key;
use axum_server::tls_rustls::RustlsConfig;
use log::info;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
use webauthn_rs::prelude::Url;
use webauthn_rs::{Webauthn, WebauthnBuilder};

pub async fn start(configuration: LoremasterWebServerConfiguration) -> Result<()> {
    let transport_layer_security_configuration: RustlsConfig = get_tls_configuration().await?;

    info!("Attempting to establish a database connection...");
    let postgres_service: PostgresHandler =
        PostgresHandler::new(configuration.database.postgresql_connection_string).await?;
    info!("Connection established.");

    info!("Creating encryption service...");
    let encryption_service: PasswordEncryptionService = PasswordEncryptionService::new(
        configuration.encryption.hash_iterations,
        configuration.encryption.site_secret.clone(),
    );

    let relaying_party_id: &str = match configuration.environment {
        crate::configuration::application::Environment::Local => LOCAL_HOST_RELAYING_PARTY_ID,
        crate::configuration::application::Environment::Development => DEV_RELAYING_PARTY_ID,
        crate::configuration::application::Environment::QualityAssurance => QA_RELAYING_PARTY_ID,
        crate::configuration::application::Environment::Production => RELAYING_PARTY_ID,
    };

    let formatted_url: String = format!("https://localhost:{}", configuration.web_server.port);
    let relaying_party_url: Url = Url::parse(formatted_url.as_str()).expect("Invalid URL");
    let web_authentication_service: Webauthn =
        WebauthnBuilder::new(relaying_party_id, &relaying_party_url)
            .expect("Invalid WebAuthn builder configuration.")
            .rp_name(RELAYING_PARTY)
            .build()
            .expect("Invalid WebAuthn builder configuration.");

    let application_state: ApplicationState = ApplicationState {
        postgres_service,
        encryption_service,
        web_authentication_service: Arc::new(web_authentication_service),
        key: Key::from(configuration.encryption.site_secret.as_bytes()),
    };

    let front_end_directory_router: MethodRouter = get_service(ServeDir::new(FRONTEND_DIST_PATH));

    info!("Configuring routers...");
    let application_router: Router = Router::new()
        .merge(api::router::authentication::router())
        .merge(api::router::chronicle::router())
        .merge(api::router::person::router())
        .with_state(application_state)
        .fallback_service(front_end_directory_router);

    let socket_address: SocketAddr = SocketAddr::from((
        configuration.web_server.ipv4_address,
        configuration.web_server.port,
    ));

    info!(
        "Starting web server...\n\n [https://localhost:{}]\n",
        configuration.web_server.port
    );
    serve(
        application_router,
        socket_address,
        transport_layer_security_configuration,
    )
    .await?;

    Ok(())
}

async fn serve(
    router: Router,
    socket_address: SocketAddr,
    transport_layer_security_configuration: RustlsConfig,
) -> Result<()> {
    axum_server::bind_rustls(socket_address, transport_layer_security_configuration)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}

async fn get_tls_configuration() -> Result<RustlsConfig> {
    Ok(RustlsConfig::from_pem_file(
        format!("{}/{}", SSL_CERT_PATH, SSL_CERT_FILE_PATH),
        format!("{}/{}", SSL_CERT_PATH, SSL_CERT_KEY_FILE_PATH),
    )
    .await?)
}

#[derive(Clone)]
pub struct ApplicationState {
    postgres_service: PostgresHandler,
    encryption_service: PasswordEncryptionService,
    web_authentication_service: Arc<Webauthn>,
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

impl FromRef<ApplicationState> for Arc<Webauthn> {
    fn from_ref(state: &ApplicationState) -> Self {
        state.web_authentication_service.clone()
    }
}
