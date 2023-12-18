use std::process::exit;

use crate::{api, kurv::Egg, printth};

use super::{Api, ParsedResponse, parse_response};
use anyhow::{Result, anyhow};
use api::eggs::EggsSummaryList;

impl Api {
    pub fn eggs_summary(&self) -> Result<EggsSummaryList> {
        let response = self.get("/eggs")?;
        let eggs_summary_list: EggsSummaryList = serde_json::from_str(&response.body)?;

        Ok(eggs_summary_list)
    }

    pub fn stop_egg(&self, id: String) -> Result<Egg> {
        let response = self.post(format!("/eggs/{}/stop", id).as_ref(), "")?;

        
        // let maybe_egg: Egg = serde_json::from_str(&response.body)?;
        let maybe_egg: ParsedResponse<Egg> = parse_response(&response)?;
        
        match maybe_egg {
            ParsedResponse::Failure(err) => {
                printth!("<error>[err: {}]</error> {}\n", err.code, err.message);
                exit(1)
            }

            ParsedResponse::Success(egg) => Ok(egg)
        }
    }
}