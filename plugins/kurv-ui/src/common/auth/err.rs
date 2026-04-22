use {
    crate::common::err::err,
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    },
    jsonwebtoken::errors::ErrorKind,
};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthError {
    /// When the token is invalid
    #[error("Invalid token")]
    InvalidToken,

    /// When the signature is invalid
    #[error("Invalid signature")]
    InvalidSignature,

    /// When a required claim is missing from the token
    #[error("Missing required claim: {0}")]
    MissingRequiredClaim(String),

    /// When a token's `exp` claim indicates that it has expired
    #[error("Expired signature")]
    ExpiredSignature,

    /// When a token's `iss` claim does not match the expected issuer
    #[error("Invalid issuer")]
    InvalidIssuer,

    /// When a token's `aud` claim does not match one of the expected audience values
    #[error("Invalid audience")]
    InvalidAudience,

    /// When a token's `sub` claim does not match one of the expected subject values
    #[error("Invalid subject")]
    InvalidSubject,

    /// When a token's `nbf` claim represents a time in the future
    #[error("Immature signature")]
    ImmatureSignature,

    /// When the algorithm in the header doesn't match the one passed to `decode` or the encoding/decoding key
    /// used doesn't match the alg requested
    #[error("Invalid algorithm")]
    InvalidAlgorithm,

    /// When the Validation struct does not contain at least 1 algorithm
    #[error("Missing algorithm")]
    MissingAlgorithm,

    /// When the request is missing the configured authentication cookie.
    #[error("Missing or invalid auth cookie")]
    MissingCookie(String),

    /// When an internal error occurs that doesn't fit into the other categories.
    /// This is a catch-all error for any unexpected errors that occur such as
    /// network errors, decoding errors, and cryptographic errors.
    #[error("Internal error")]
    InternalError,
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(val: jsonwebtoken::errors::Error) -> Self {
        match val.kind() {
            ErrorKind::InvalidToken => AuthError::InvalidToken,
            ErrorKind::InvalidSignature => AuthError::InvalidSignature,
            ErrorKind::MissingRequiredClaim(claim) => {
                AuthError::MissingRequiredClaim(claim.to_string())
            }
            ErrorKind::ExpiredSignature => AuthError::ExpiredSignature,
            ErrorKind::InvalidIssuer => AuthError::InvalidIssuer,
            ErrorKind::InvalidAudience => AuthError::InvalidAudience,
            ErrorKind::InvalidSubject => AuthError::InvalidSubject,
            ErrorKind::ImmatureSignature => AuthError::ImmatureSignature,
            ErrorKind::InvalidAlgorithm => AuthError::InvalidAlgorithm,
            ErrorKind::MissingAlgorithm => AuthError::MissingAlgorithm,
            _ => AuthError::InternalError,
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AuthError::InvalidToken => err(StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::InvalidSignature => err(StatusCode::UNAUTHORIZED, "Invalid signature"),
            AuthError::MissingRequiredClaim(_) => {
                err(StatusCode::UNAUTHORIZED, "Missing required claim")
            }
            AuthError::ExpiredSignature => err(StatusCode::UNAUTHORIZED, "Expired signature"),
            AuthError::InvalidIssuer => err(StatusCode::UNAUTHORIZED, "Invalid issuer"),
            AuthError::InvalidAudience => err(StatusCode::UNAUTHORIZED, "Invalid audience"),
            AuthError::InvalidSubject => err(StatusCode::UNAUTHORIZED, "Invalid subject"),
            AuthError::ImmatureSignature => err(StatusCode::UNAUTHORIZED, "Immature signature"),
            AuthError::InvalidAlgorithm => err(StatusCode::UNAUTHORIZED, "Invalid algorithm"),
            AuthError::MissingAlgorithm => err(StatusCode::UNAUTHORIZED, "Missing algorithm"),
            AuthError::MissingCookie(_) => {
                err(StatusCode::UNAUTHORIZED, "Missing or invalid auth cookie")
            }
            AuthError::InternalError => err(StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };

        (status, msg).into_response()
    }
}
