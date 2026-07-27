use {
    crate::app::kurv_ui::KurvUIConfig,
    axum::{
        extract::{Request, State},
        http::{StatusCode, Uri, header},
        response::{Html, IntoResponse, Response},
    },
    rust_embed::Embed,
};

mod globals;
mod mime;

static INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "frontend/dist/"]
struct Assets;

pub async fn frontend_handler(
    uri: Uri,
    State(config): State<KurvUIConfig>,
    request: Request,
) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    // inject server-globals.js in runtime
    if path == "server-globals.js" {
        return (
            [(header::CONTENT_TYPE, "application/javascript")],
            globals::get_globals_js(&config, &request),
        )
            .into_response();
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime::guess(path);
            ([(header::CONTENT_TYPE, mime)], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
