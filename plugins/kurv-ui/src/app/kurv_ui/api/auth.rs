use {
    crate::{
        app::kurv_ui::{
            KurvAppContext, extractors::auth_user::AuthedUser, services::users::UsersService,
        },
        common::{
            auth::jwt::codec::Claims,
            err::{self, AppErr},
            extractor::json::Json,
        },
    },
    axum::{Router, extract::State, routing},
    chrono::{Duration, Utc},
    jsonwebtoken::EncodingKey,
    serde::Deserialize,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub fn router() -> Router<KurvAppContext> {
    Router::new()
        .route("/login", routing::post(post::login))
        .route("/me", routing::get(get::get_current))
}

pub mod get {
    use super::*;

    /// Get Logged-in Entity
    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn get_current(authed_user: AuthedUser) -> Result<Json<AuthedUser>, AppErr> {
        Ok(Json(authed_user))
    }
}

pub mod post {
    use super::*;

    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginResponse {
        pub access_token: String,
        pub schema: String,
    }

    /// Login Endpoint
    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn login(
        State(context): State<KurvAppContext>,
        users_service: UsersService,
        Json(payload): Json<LoginRequest>,
    ) -> Result<Json<LoginResponse>, AppErr> {
        // validate the username and password
        let user = users_service.verify(&payload.username, &payload.password).await?;

        if let Some(user) = user {
            // if the password is correct, we create a JWT token
            let now = Utc::now();

            let claims = Claims {
                iat: now.timestamp() as u64,
                exp: now
                    .checked_add_signed(Duration::seconds(
                        context.config.security.jwt.token_expiration,
                    ))
                    .ok_or_else(|| {
                        err::internal_error("Failed to calculate token expiration time")
                    })?
                    .timestamp() as u64,
                sub: user.username,
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &EncodingKey::from_secret(context.config.security.jwt.secret.as_bytes()),
            )
            .map_err(err::internal_error)?;

            Ok(Json(LoginResponse {
                access_token: token,
                schema: context.config.security.jwt.token_schema,
            }))
        } else {
            Err(err::unauthorized("Invalid username or password"))
        }
    }
}
