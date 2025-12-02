use {
    super::{Api, ParsedResponse, parse_response},
    crate::{api, kurv::Egg, printth},
    anyhow::Result,
    api::eggs::EggsSummaryList,
    std::{collections::HashMap, process::exit},
};

#[derive(Debug, PartialEq, Eq)]
pub enum EggKind {
    Eggs,
    Plugins,
}

impl EggKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            EggKind::Eggs => "eggs",
            EggKind::Plugins => "plugins",
        }
    }

    pub fn as_display_str(&self) -> &'static str {
        match self {
            EggKind::Eggs => "⬮",
            EggKind::Plugins => "plugin",
        }
    }
}

impl Api {
    pub fn eggs_summary(&self, kind: &EggKind) -> Result<EggsSummaryList> {
        let response = self.get(format!("/eggs?kind={}", kind.as_str()).as_ref())?;
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

    /// update egg environment variables, either merging or replacing them
    pub fn update_egg_env(
        &self,
        id: &str,
        env: &HashMap<String, String>,
        replace: bool,
    ) -> Result<Egg> {
        // merge: HTTP PATCH /eggs/{id}/env
        // replace: HTTP PUT /eggs/{id}/env

        let body = serde_json::to_string(&env)?;

        let response = if replace {
            self.put(format!("/eggs/{}/env", id).as_ref(), body.as_str())?
        } else {
            self.patch(format!("/eggs/{}/env", id).as_ref(), body.as_str())?
        };

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
