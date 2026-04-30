use {
    crate::{
        app::kurv_ui::{KurvAppContext, services::users::UsersService},
        common::auth::{
            err::AuthError,
            jwt::{codec::Claims, extractor::JwtCodecState},
        },
    },
    axum::{extract::FromRef, http::request::Parts},
    serde::Serialize,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthedUser {
    pub username: String,
}

impl<S> axum::extract::FromRequestParts<S> for AuthedUser
where
    KurvAppContext: FromRef<S>,
    JwtCodecState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = KurvAppContext::from_ref(state);

        let claims = Claims::from_request_parts(parts, state).await?;
        let db = UsersService::from_request_parts(parts, &app_state)
            .await
            .map_err(|_| AuthError::InternalError)?;

        let maybe_user =
            db.get_by_username(&claims.sub).await.map_err(|_| AuthError::InternalError)?;

        match maybe_user {
            Some(user) => Ok(AuthedUser {
                username: user.username,
            }),
            None => Err(AuthError::InvalidSubject),
        }
    }
}
