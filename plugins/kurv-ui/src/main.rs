use {
    axum_extra::extract::cookie::SameSite,
    kurv_plugin_sdk::{KurvEnv, PluginConfig, discover_env, plugin_metadata, start_async},
    kurv_ui::{KurvUi, app::kurv_ui::KurvUIEnv},
    log,
    std::{env, path::Path},
};

#[tokio::main]
async fn main() {
    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Trace);
    clog.init();

    start_async(
        plugin_metadata!(),
        |exe| {
            let env = discover_env(exe).expect("kurv-ui: failed to load sidecar config");

            PluginConfig {
                name: "kurv-ui".into(),
                command: exe.to_string_lossy().into_owned(),
                args: vec!["run".into()],
                env,
                ..Default::default()
            }
        },
        run,
    )
}

async fn run(env: KurvEnv) {
    log::info!("initializing kurv-ui...");

    let kurv_ui_env = match build_kurv_ui_env(&env) {
        Ok(kurv_ui_env) => kurv_ui_env,
        Err(err) => {
            log::error!("failed to load kurv-ui configuration: {err}");
            return;
        }
    };

    log::trace!("KURV_API_HOST: {}", env.api_host);
    log::trace!("KURV_API_PORT: {}", env.api_port);
    log::trace!("KURV_HOME:     {}", env.home.display());
    log::trace!("KURV_LOGS_DIR: {}", env.logs_dir.display());
    log::info!("KURV_UI_HOST: {}", kurv_ui_env.host);
    log::info!("KURV_UI_PORT: {}", kurv_ui_env.port);
    log::info!("KURV_UI_DB_URL: {}", kurv_ui_env.db_url);
    log::info!("KURV_UI_JWT_EXPIRATION: {}", kurv_ui_env.security_jwt_expiration);
    log::info!("KURV_UI_COOKIE_NAME: {}", kurv_ui_env.security_cookie_name);
    log::info!("KURV_UI_COOKIE_SECURE: {}", kurv_ui_env.security_cookie_secure);
    log::info!(
        "KURV_UI_COOKIE_SAME_SITE: {}",
        same_site_to_env_value(kurv_ui_env.security_cookie_same_site)
    );

    match KurvUi::new(kurv_ui_env, env).await {
        Ok(app) => {
            if let Err(err) = app.serve().await {
                log::error!("kurv-ui server stopped with an error: {err}");
            }
        }
        Err(err) => {
            log::error!("failed to initialize kurv-ui: {err}");
        }
    }
}

fn build_kurv_ui_env(env: &KurvEnv) -> Result<KurvUIEnv, String> {
    Ok(KurvUIEnv {
        host: envvar("KURV_UI_HOST", "127.0.0.1"),
        port: envvar("KURV_UI_PORT", "9500")
            .parse()
            .map_err(|err| format!("invalid KURV_UI_PORT: {err}"))?,
        db_url: envvar("KURV_UI_DB_URL", &default_db_url(&env.home)),
        security_jwt_secret: envvar("KURV_UI_JWT_SECRET", "change-me"),
        security_jwt_expiration: envvar("KURV_UI_JWT_EXPIRATION", "3600")
            .parse()
            .map_err(|err| format!("invalid KURV_UI_JWT_EXPIRATION: {err}"))?,
        security_cookie_name: envvar("KURV_UI_COOKIE_NAME", "kurv_ui_session"),
        security_cookie_secure: envvar_bool("KURV_UI_COOKIE_SECURE", false)?,
        security_cookie_same_site: envvar_same_site("KURV_UI_COOKIE_SAME_SITE", SameSite::Lax)?,
    })
}

fn envvar(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn envvar_bool(key: &str, default: bool) -> Result<bool, String> {
    match env::var(key) {
        Ok(value) => match value.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            _ => Err(format!("invalid {key}: expected a boolean value")),
        },
        Err(_) => Ok(default),
    }
}

fn envvar_same_site(key: &str, default: SameSite) -> Result<SameSite, String> {
    match env::var(key) {
        Ok(value) => match value.to_ascii_lowercase().as_str() {
            "strict" => Ok(SameSite::Strict),
            "lax" => Ok(SameSite::Lax),
            "none" => Ok(SameSite::None),
            _ => Err(format!("invalid {key}: expected one of strict, lax, none")),
        },
        Err(_) => Ok(default),
    }
}

fn same_site_to_env_value(same_site: SameSite) -> &'static str {
    match same_site {
        SameSite::Strict => "strict",
        SameSite::Lax => "lax",
        SameSite::None => "none",
    }
}

fn default_db_url(home: &Path) -> String {
    let path = home.join("kurv-ui.sqlite");
    let path = path.to_string_lossy().replace('\\', "/");
    format!("sqlite://{path}?mode=rwc")
}
