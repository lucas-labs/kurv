// placeholder for kurv managing api routes and handlers
// endpoints here will communicate with the kurv server's HTTP API to manage eggs; instead of
// going directly from the UI to the kurv server, we will have this kurv-ui API as an intermediary
// because the server might be running somewhere out of reach for the clients of the UI; so
// from this plugin we will treat the kurv server as an external server and kurv-ui apis will
// encapsulate the logic of communicating with kurv.

use {
    crate::{
        app::kurv_ui::{
            KurvAppContext,
            extractors::auth_user::AuthedUser,
            services::kurv_api::{KurvApiService, KurvEgg, KurvEggSummaryList},
        },
        common::{err::AppErr, extractor::json::Json},
    },
    axum::{
        Router,
        extract::{Path, Query},
        routing,
    },
    serde::Deserialize,
};

pub fn router() -> Router<KurvAppContext> {
    Router::new()
        .route("/eggs", routing::get(get::list_eggs))
        .route("/eggs/{egg_id}", routing::get(get::get_egg))
        .route("/eggs/{egg_id}/start", routing::post(post::start_egg))
        .route("/eggs/{egg_id}/stop", routing::post(post::stop_egg))
        .route("/eggs/{egg_id}/restart", routing::post(post::restart_egg))
}

#[derive(Default, Deserialize)]
pub struct ListEggsQuery {
    pub kind: Option<String>,
}

pub mod get {
    use super::*;

    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn list_eggs(
        _authed_user: AuthedUser,
        kurv_api: KurvApiService,
        Query(query): Query<ListEggsQuery>,
    ) -> Result<Json<KurvEggSummaryList>, AppErr> {
        let eggs = kurv_api.list_eggs(query.kind.as_deref()).await?;

        Ok(Json(eggs))
    }

    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn get_egg(
        _authed_user: AuthedUser,
        kurv_api: KurvApiService,
        Path(egg_id): Path<String>,
    ) -> Result<Json<KurvEgg>, AppErr> {
        let egg = kurv_api.get_egg(&egg_id).await?;

        Ok(Json(egg))
    }
}

pub mod post {
    use super::*;

    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn start_egg(
        _authed_user: AuthedUser,
        kurv_api: KurvApiService,
        Path(egg_id): Path<String>,
    ) -> Result<Json<KurvEgg>, AppErr> {
        let egg = kurv_api.start_egg(&egg_id).await?;

        Ok(Json(egg))
    }

    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn stop_egg(
        _authed_user: AuthedUser,
        kurv_api: KurvApiService,
        Path(egg_id): Path<String>,
    ) -> Result<Json<KurvEgg>, AppErr> {
        let egg = kurv_api.stop_egg(&egg_id).await?;

        Ok(Json(egg))
    }

    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn restart_egg(
        _authed_user: AuthedUser,
        kurv_api: KurvApiService,
        Path(egg_id): Path<String>,
    ) -> Result<Json<KurvEgg>, AppErr> {
        let egg = kurv_api.restart_egg(&egg_id).await?;

        Ok(Json(egg))
    }
}
