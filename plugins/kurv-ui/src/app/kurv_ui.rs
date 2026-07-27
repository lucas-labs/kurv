mod api;
mod services;
mod extractors {
    pub mod auth_user;
}

use {
    crate::{
        app::frontend,
        common::{
            auth::jwt::{codec::JwtCodec, extractor::JwtCodecState},
            middleware,
        },
        db::models::get_db,
    },
    axum::{Router, extract::FromRef},
    axum_extra::extract::cookie::SameSite,
    eyre::{Result, eyre},
    kurv_plugin_sdk::KurvEnv,
    log::info,
    sea_orm::{ConnectionTrait, DatabaseConnection},
    services::setup::{SetupService, SetupStatus},
    std::{
        net::SocketAddr,
        sync::{Arc, Mutex},
    },
    tokio::net::TcpListener,
};

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub token_expiration: i64, // in seconds
}

#[derive(Clone)]
pub struct CookieConfig {
    pub name: String,
    pub secure: bool,
    pub same_site: SameSite,
    pub max_age: i64,
}

#[derive(Clone)]
pub struct SecurityConfig {
    pub jwt: JwtConfig,
    pub cookie: CookieConfig,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub version: String,
}

#[derive(Clone)]
pub struct KurvUIConfig {
    pub security: SecurityConfig,
    pub server: ServerConfig,
}

#[derive(Clone, FromRef)]
pub struct KurvAppContext {
    pub env: KurvEnv,
    pub setup_status: Arc<Mutex<SetupStatus>>,
    pub db: DatabaseConnection,
    pub config: KurvUIConfig,
    pub codec: JwtCodecState,
}

pub struct KurvUIEnv {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub security_jwt_secret: String,
    pub security_jwt_expiration: i64,
    pub security_cookie_name: String,
    pub security_cookie_secure: bool,
    pub security_cookie_same_site: SameSite,
}

pub struct KurvUi {
    /// environment injected by the kurv instance, with runtime info (kurv host, port, etc)
    kurv_env: KurvEnv,
    /// environment for kurv-ui itself, with config for the kurv-ui server
    kurv_ui_env: KurvUIEnv,
}

impl KurvUi {
    pub async fn new(kurv_ui_env: KurvUIEnv, kurv_env: KurvEnv) -> Result<Self> {
        info!("Initializing KurvUi...");
        Ok(Self {
            kurv_env,
            kurv_ui_env,
        })
    }

    pub async fn serve(&self) -> Result<()> {
        let db = get_db(&self.kurv_ui_env.db_url, false).await?;
        let server_addr = format!("{}:{}", self.kurv_ui_env.host, self.kurv_ui_env.port);
        let jwt_codec = Arc::new(JwtCodec::new(&self.kurv_ui_env.security_jwt_secret));
        let setup = SetupService { db: db.clone() };

        // Setup
        setup.migrate().await?;
        let setup_status = setup.get_status().await?;

        let context = KurvAppContext {
            env: self.kurv_env.clone(),
            db: db.clone(),
            setup_status: Arc::new(Mutex::new(setup_status)),
            config: KurvUIConfig {
                security: SecurityConfig {
                    jwt: JwtConfig {
                        secret: self.kurv_ui_env.security_jwt_secret.clone(),
                        token_expiration: self.kurv_ui_env.security_jwt_expiration,
                    },
                    cookie: CookieConfig {
                        name: self.kurv_ui_env.security_cookie_name.clone(),
                        secure: self.kurv_ui_env.security_cookie_secure,
                        same_site: self.kurv_ui_env.security_cookie_same_site,
                        max_age: self.kurv_ui_env.security_jwt_expiration,
                    },
                },
                server: ServerConfig {
                    host: self.kurv_ui_env.host.clone(),
                    port: self.kurv_ui_env.port,
                    name: "kurv-ui".into(),
                    version: env!("CARGO_PKG_VERSION").into(),
                },
            },
            codec: JwtCodecState { codec: jwt_codec },
        };

        // Create the OpenAPI router with all routes included
        let app = Router::new();

        let app = app
            .nest("/api", api::routes())
            .fallback(frontend::frontend_handler) // serve frontend as a fallback
            .layer(middleware::stack()) // cors, default headers, etc
            .with_state(context)
            .into_make_service_with_connect_info::<SocketAddr>();

        let listener = TcpListener::bind(&server_addr).await?;
        info!("Listening on {}", &server_addr);

        // start the server with graceful shutdown
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| eyre!("Failed to start server: {}", e))?;

        // Ensure database connections are properly closed
        info!("Performing final database cleanup...");
        if let Err(e) = db.execute_unprepared("PRAGMA wal_checkpoint(TRUNCATE);").await {
            eprintln!("Warning: Failed to checkpoint WAL during shutdown: {}", e);
        }

        Ok(())
    }
}

async fn shutdown_signal() {
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).recv().await;
    };

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully...");
        },
        _ = terminate => {
            info!("Received terminate signal, shutting down gracefully...");
        },
    }
}
