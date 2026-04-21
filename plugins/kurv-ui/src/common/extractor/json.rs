use {
    crate::common::err::ErrorResponse,
    axum::{
        extract::{FromRequest, rejection::JsonRejection},
        http::StatusCode,
        response::IntoResponse,
    },
    serde::Serialize,
};

/// JSON Extractor / Response.
///
/// Custom JSON  extractor. Same as [`axum::Json`], but instead of generating a raw text response,
/// this one generates an `application/json` response with the shape of [`ErrorResponse`].
///
/// See [`axum::Json`] for more details.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ErrorResponse))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

impl From<JsonRejection> for ErrorResponse {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            code: rejection.status().as_u16(),
            message: rejection.body_text(),
        }
    }
}

// we implement `IntoResponse` so `ErrorResponse` can be used as a response
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, axum::Json(self)).into_response()
    }
}
