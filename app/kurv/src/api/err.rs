use {
    super::Context,
    crate::common::tcp::{ErrorResponse, Request, Response, json},
    anyhow::Result,
};

/// handle not allowed requests
pub fn not_allowed(_request: &Request, _ctx: &Context) -> Result<Response> {
    Ok(json(
        405,
        ErrorResponse {
            code: 405,
            status: "Method Not Allowed".to_string(),
            message: "The method specified in the request is not allowed.".to_string(),
        },
    ))
}
