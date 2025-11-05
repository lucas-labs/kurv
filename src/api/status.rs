use {
    super::Context,
    crate::common::tcp::{Request, Response, json},
    anyhow::Result,
};

pub fn status(_request: &Request, ctx: &Context) -> Result<Response> {
    let info = ctx.info.clone();
    let info = info.lock().unwrap();

    Ok(json(200, info.clone()))
}
