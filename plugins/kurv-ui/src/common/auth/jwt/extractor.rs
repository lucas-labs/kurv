use {
    crate::{
        app::kurv_ui::KurvAppContext,
        common::auth::{
            err::AuthError,
            jwt::codec::{Claims, JwtCodec},
        },
    },
    axum::{extract::FromRef, http::request::Parts},
    axum_extra::extract::cookie::CookieJar,
    std::sync::Arc,
};

/// Extracts a JWT token from the configured auth cookie.
fn extract_token(parts: &Parts, cookie_name: &str) -> Result<String, AuthError> {
    let jar = CookieJar::from_headers(&parts.headers);
    let cookie = jar
        .get(cookie_name)
        .ok_or_else(|| AuthError::MissingCookie(cookie_name.to_string()))?;

    Ok(cookie.value().to_owned())
}

impl<S> axum::extract::FromRequestParts<S> for Claims
where
    KurvAppContext: FromRef<S>,
    JwtCodecState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let context = KurvAppContext::from_ref(state);
        let token = extract_token(parts, &context.config.security.cookie.name)?;
        let state = JwtCodecState::from_ref(state);
        let token_data = state.codec.decode(&token).await?;

        Ok(token_data.claims)
    }
}

#[derive(Clone)]
pub struct JwtCodecState {
    pub codec: Arc<JwtCodec>,
}
