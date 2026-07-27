use {
    axum::{Json, http::StatusCode},
    log::error,
    serde::Serialize,
};
pub type AppErr = (StatusCode, Json<ErrorResponse>);

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

pub fn err(code: StatusCode, message: impl std::fmt::Display) -> AppErr {
    let mut message = message.to_string();

    error!("Error: {} - {}", code, message);

    if code == StatusCode::INTERNAL_SERVER_ERROR {
        message = "Internal server error".to_string();
    }

    (
        code,
        Json(ErrorResponse {
            code: code.as_u16(),
            message,
        }),
    )
}

pub fn not_found(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::NOT_FOUND, message)
}

pub fn unauthorized(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::UNAUTHORIZED, message)
}

pub fn forbidden(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::FORBIDDEN, message)
}

pub fn internal_error(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::INTERNAL_SERVER_ERROR, message)
}

pub fn bad_request(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::BAD_REQUEST, message)
}

pub fn conflict(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::CONFLICT, message)
}

pub fn unprocessable_entity(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::UNPROCESSABLE_ENTITY, message)
}

pub fn not_implemented(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::NOT_IMPLEMENTED, message)
}

pub fn im_a_teapot(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::IM_A_TEAPOT, message)
}

pub fn unsupported_media_type(message: impl std::fmt::Display) -> AppErr {
    err(StatusCode::UNSUPPORTED_MEDIA_TYPE, message)
}
