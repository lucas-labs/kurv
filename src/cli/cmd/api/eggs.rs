use crate::api;

use super::Api;
use anyhow::Result;
use api::eggs::EggsSummaryList;

impl Api {
    pub fn eggs_summary(&self) -> Result<EggsSummaryList> {
        let response = self.get("/eggs")?;
        let eggs_summary_list: EggsSummaryList = serde_json::from_str(&response.body)?;

        Ok(eggs_summary_list)
    }
}