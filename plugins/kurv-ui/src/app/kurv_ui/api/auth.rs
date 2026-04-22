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
    axum::{Router, extract::State, http::StatusCode, routing},
    axum_extra::extract::cookie::{Cookie, CookieJar},
    chrono::{Duration, Utc},
    jsonwebtoken::EncodingKey,
    serde::Deserialize,
    time::Duration as CookieDuration,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub fn router() -> Router<KurvAppContext> {
    Router::new()
        .route("/login", routing::post(post::login))
    .route("/logout", routing::post(post::logout))
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

    fn build_auth_cookie(context: &KurvAppContext, token: String) -> Cookie<'static> {
        let mut cookie = Cookie::new(context.config.security.cookie.name.clone(), token);
        cookie.set_http_only(true);
        cookie.set_max_age(CookieDuration::seconds(context.config.security.cookie.max_age));
        cookie.set_path("/");
        cookie.set_same_site(context.config.security.cookie.same_site);
        cookie.set_secure(context.config.security.cookie.secure);
        cookie
    }

    fn build_expired_auth_cookie(context: &KurvAppContext) -> Cookie<'static> {
        let mut cookie = Cookie::new(context.config.security.cookie.name.clone(), String::new());
        cookie.set_http_only(true);
        cookie.set_max_age(CookieDuration::seconds(0));
        cookie.set_path("/");
        cookie.set_same_site(context.config.security.cookie.same_site);
        cookie.set_secure(context.config.security.cookie.secure);
        cookie
    }

    /// Login Endpoint
    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn login(
        State(context): State<KurvAppContext>,
        jar: CookieJar,
        users_service: UsersService,
        Json(payload): Json<LoginRequest>,
    ) -> Result<(CookieJar, Json<AuthedUser>), AppErr> {
        // validate the username and password
        let user = users_service.verify(&payload.username, &payload.password).await?;

        if let Some(user) = user {
            let username = user.username;

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
                sub: username.clone(),
            };

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &EncodingKey::from_secret(context.config.security.jwt.secret.as_bytes()),
            )
            .map_err(err::internal_error)?;

            Ok((
                jar.add(build_auth_cookie(&context, token)),
                Json(AuthedUser { username }),
            ))
        } else {
            Err(err::unauthorized("Invalid username or password"))
        }
    }

    /// Logout Endpoint
    #[axum::debug_handler(state=KurvAppContext)]
    pub async fn logout(
        State(context): State<KurvAppContext>,
        jar: CookieJar,
    ) -> Result<(CookieJar, StatusCode), AppErr> {
        Ok((
            jar.add(build_expired_auth_cookie(&context)),
            StatusCode::NO_CONTENT,
        ))
    }
}
