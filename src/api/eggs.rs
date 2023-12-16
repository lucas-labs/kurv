use std::collections::BTreeMap;

use crate::{common::tcp::{json, Request, Response}, kurv::egg::Egg};
use anyhow::{anyhow, Result};
use serde::Serialize;

use super::RouterHandler;

#[derive(Serialize)]
struct ListEggsResponse(BTreeMap<String, Egg>);

impl RouterHandler {
    pub fn list_eggs(&self, _request: &Request) -> Result<Response> {
        let state = self.state.clone();
        let state = state.lock().map_err(|_| anyhow!("failed to lock state"))?;

        let eggs = state.eggs.clone();


        Ok(json(
            200,
            ListEggsResponse(eggs),
        ))
    }
}
