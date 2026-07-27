pub mod auth;
pub mod kurv;
pub mod setup;

use {crate::app::kurv_ui::KurvAppContext, axum::Router};

pub fn routes() -> Router<KurvAppContext> {
    Router::<KurvAppContext>::new() // api routes
        .nest("/auth", auth::router())
        .nest("/setup", setup::router())
        .nest("/kurv", kurv::router())
}
