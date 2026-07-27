use {
    crate::{
        app::kurv_ui::{
            KurvAppContext,
            services::{setup::SetupStatus, users::UsersService},
        },
        common::{
            err::{self, AppErr},
            extractor::json::Json,
        },
    },
    axum::{Router, extract::State, routing},
    std::sync::{Arc, Mutex},
};

pub fn router() -> Router<KurvAppContext> {
    Router::new() // admin setup routes
        .route("/status", routing::get(get::setup_status))
        .route("/initial-user", routing::post(post::setup))
}

pub mod get {

    use super::*;

    #[derive(serde::Serialize)]
    pub struct StatusResponse {
        pub status: String,
    }

    /// Get setup status
    ///
    /// Returns the current setup status of the application;
    /// - `uninitialized`: Setup has not been completed.
    /// - `ready`: Setup is complete and the initial admin user has been created.

    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn setup_status(
        State(status): State<Arc<Mutex<SetupStatus>>>,
    ) -> Result<Json<StatusResponse>, AppErr> {
        let status = {
            let status = status.lock().map_err(err::internal_error)?;
            status.clone()
        };

        Ok(Json(StatusResponse {
            status: status.to_string(),
        }))
    }
}

pub mod post {

    use super::*;

    #[derive(serde::Deserialize)]
    pub struct SetupRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(serde::Serialize)]
    pub struct SetupResponse {
        pub username: String,
    }

    /// Create initial user
    ///
    /// Creates the initial admin user for the application.
    ///
    /// This endpoint should only be called once when setting up the application.
    /// If the setup is already complete, it will return an error.
    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn setup(
        State(status): State<Arc<Mutex<SetupStatus>>>,
        user_svc: UsersService,
        Json(request): Json<SetupRequest>,
    ) -> Result<Json<SetupResponse>, AppErr> {
        let current_status = {
            let status = status.lock().map_err(err::internal_error)?;
            status.clone()
        };

        if current_status != SetupStatus::Uninitialized {
            return Err(err::unprocessable_entity(
                "Setup is already complete. Initial user has been created.",
            ));
        }

        // create the initial user
        let user = user_svc.create(&request.username, &request.password).await?;

        // uUpdate the setup status to complete
        {
            let mut status = status.lock().map_err(err::internal_error)?;
            *status = SetupStatus::Ready;
        }

        Ok(Json(SetupResponse {
            username: user.username,
        }))
    }
}
