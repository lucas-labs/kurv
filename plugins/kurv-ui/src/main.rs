use {
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
            let mut env = discover_env(exe).expect("kurv-ui: failed to load sidecar config");
            env.insert("HELLO_MESSAGE".into(), "Hello from kurv-ui plugin!".into());

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
    log::trace!("KURV_API_HOST: {}", env.api_host);
    log::trace!("KURV_API_PORT: {}", env.api_port);
    log::trace!("KURV_HOME:     {}", env.home.display());
    log::trace!("KURV_LOGS_DIR: {}", env.logs_dir.display());

    let kurv_ui_env = match build_kurv_ui_env(&env) {
        Ok(kurv_ui_env) => kurv_ui_env,
        Err(err) => {
            log::error!("failed to load kurv-ui configuration: {err}");
            return;
        }
    };

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
        security_jwt_schema: envvar("KURV_UI_JWT_SCHEMA", "Bearer"),
    })
}

fn envvar(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_owned())
}

fn default_db_url(home: &Path) -> String {
    let path = home.join("kurv-ui.sqlite");
    let path = path.to_string_lossy().replace('\\', "/");
    format!("sqlite://{path}?mode=rwc")
}
