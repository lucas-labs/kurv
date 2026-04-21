use {crate::app::kurv_ui::KurvUIConfig, axum::extract::Request, log::info, std::sync::RwLock};

static GLOBAL_HOST: RwLock<String> = RwLock::new(String::new());
static GLOBAL_JAVASCRIPT: RwLock<String> = RwLock::new(String::new());

fn get_host(request: &Request) -> String {
    request.headers().get("host").and_then(|h| h.to_str().ok()).unwrap_or("/api").to_string()
}

fn initialize(config: &KurvUIConfig, host: String) {
    info!("Initializing globals with host: {}", host);

    let value = format!(
        "window.__SERVER__={{BASE_URL:'{}/api',VERSION:'{}'}};",
        host, config.server.version,
    );

    {
        let mut js = GLOBAL_JAVASCRIPT.write().unwrap();
        *js = value;
    }

    {
        let mut h = GLOBAL_HOST.write().unwrap();
        *h = host;
    }
}

pub fn get_globals_js(config: &KurvUIConfig, request: &Request) -> &'static str {
    let host = get_host(request);
    let current_host = GLOBAL_HOST.read().unwrap().clone();

    if host != current_host {
        initialize(config, host);
    }

    Box::leak(GLOBAL_JAVASCRIPT.read().unwrap().clone().into_boxed_str())
}
