use crate::api;
use crate::utility::constants::files::FRONTEND_DIST_PATH;
use crate::utility::constants::{ENVIRONMENT, LOCAL_HOST_RELAYING_PARTY_ID, RELAYING_PARTY};
use crate::utility::loremaster_configuration::{
    get_configuration_from_file, LoremasterConfiguration,
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

pub async fn start() -> Result<()> {
    let environment: String = std::env::var(ENVIRONMENT)?;
    let configuration: LoremasterConfiguration = get_configuration_from_file(&environment)?;

    let transport_layer_security_configuration: RustlsConfig = get_tls_configuration().await?;

    info!("Attempting to establish a database connection...");
    let postgres_service: PostgresHandler =
        PostgresHandler::new(configuration.postgresql_connection_string).await?;
    info!("Connection established.");

    info!("Creating encryption service...");
    let encryption_service: PasswordEncryptionService = PasswordEncryptionService::new(
        configuration.hash_iterations,
        configuration.site_secret.clone(),
    );

    //TODO: match environment
    let relaying_party_url: Url =
        Url::parse("http://localhost:{configuration.port}").expect("Invalid URL");
    let web_authentication_service: Webauthn =
        WebauthnBuilder::new(LOCAL_HOST_RELAYING_PARTY_ID, &relaying_party_url)
            .expect("Invalid WebAuthn builder configuration.")
            .rp_name(RELAYING_PARTY)
            .build()
            .expect("Invalid WebAuthn builder configuration.");

    let application_state: ApplicationState = ApplicationState {
        postgres_service,
        encryption_service,
        web_authentication_service: Arc::new(web_authentication_service),
        key: Key::from(configuration.site_secret.as_bytes()),
    };

    let front_end_directory_router: MethodRouter = get_service(ServeDir::new(FRONTEND_DIST_PATH));

    info!("Configuring routers...");
    let application_router: Router = Router::new()
        .merge(api::router::authentication::router())
        .merge(api::router::chronicle::router())
        .merge(api::router::person::router())
        .with_state(application_state)
        .fallback_service(front_end_directory_router);

    let socket_address: SocketAddr =
        SocketAddr::from((configuration.ipv4_address, configuration.port));

    info!(
        "Starting web server...\n\n [https://localhost:{}]\n",
        configuration.port
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
    Ok(RustlsConfig::from_pem_file("certs/cert.pem", "certs/key.pem").await?)
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
