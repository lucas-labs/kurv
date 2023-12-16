use {
    super::RouterHandler,
    crate::common::tcp::{json, Request, Response},
    anyhow::Result,
};

impl RouterHandler {
    pub fn status(&self, _request: &Request) -> Result<Response> {
        let info = self.info.clone();
        let info = info.lock().unwrap();

        Ok(json(200, info.clone()))
    }
}
