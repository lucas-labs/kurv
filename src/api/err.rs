use crate::common::tcp::{json, ErrorResponse, Request, Response};
use anyhow::Result;

use super::RouterHandler;

/// handle not allowed requests
impl RouterHandler {
    pub fn not_allowed(&self, _request: &Request) -> Result<Response> {
        Ok(json(405, ErrorResponse {
            code: 405,
            status: "Method Not Allowed".to_string(),
            message: "The method specified in the Request-Line is not allowed for the resource identified by the Request-URI.".to_string()
        }))
    }
}
