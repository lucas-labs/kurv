use {
    crate::common::auth::{
        err::AuthError,
        jwt::codec::{Claims, JwtCodec},
    },
    axum::{RequestPartsExt, extract::FromRef, http::request::Parts},
    axum_extra::{
        TypedHeader,
        headers::{Authorization, authorization::Bearer},
    },
    std::sync::Arc,
};

/// Extracts a JWT Bearer token from the request Authorization header
async fn extract_token(parts: &mut Parts) -> Result<String, AuthError> {
    let auth: TypedHeader<Authorization<Bearer>> =
        parts.extract().await.map_err(|_| AuthError::MissingToken)?;

    Ok(auth.token().to_string())
}

impl<S> axum::extract::FromRequestParts<S> for Claims
where
    JwtCodecState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = extract_token(parts).await?;
        let state = JwtCodecState::from_ref(state);
        let token_data = state.codec.decode(&token).await?;

        Ok(token_data.claims)
    }
}

#[derive(Clone)]
pub struct JwtCodecState {
    pub codec: Arc<JwtCodec>,
}
