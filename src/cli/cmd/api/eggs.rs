use {
    super::{Api, ParsedResponse, parse_response},
    crate::{api, kurv::Egg, printth},
    anyhow::Result,
    api::eggs::EggsSummaryList,
    std::process::exit,
};

impl Api {
    pub fn eggs_summary(&self) -> Result<EggsSummaryList> {
        let response = self.get("/eggs")?;
        let eggs_summary_list: EggsSummaryList = serde_json::from_str(&response.body)?;

        Ok(eggs_summary_list)
    }

    pub fn egg(&self, id: &str) -> Result<Egg> {
        let response = self.get(format!("/eggs/{}", id).as_ref())?;
        let maybe_egg: ParsedResponse<Egg> = parse_response(&response)?;

        match maybe_egg {
            ParsedResponse::Failure(err) => {
                printth!("<error>[err: {}]</error> {}\n", err.code, err.message);
                exit(1)
            }

            ParsedResponse::Success(egg) => Ok(egg),
        }
    }

    pub fn eggs_post(&self, route: &str, body: &str) -> Result<Egg> {
        let response = self.post(format!("/eggs{route}").as_ref(), body)?;
        let maybe_egg: ParsedResponse<Egg> = parse_response(&response)?;

        match maybe_egg {
            ParsedResponse::Failure(err) => {
                printth!("<error>[err: {}]</error> {}\n", err.code, err.message);
                exit(1)
            }

            ParsedResponse::Success(egg) => Ok(egg),
        }
    }
}
