use crate::common::duration::humanize_duration;

use {
    crate::common::tcp::{json, Request, Response},
    crate::kurv::egg::EggStatus,
    super::Context,
    super::err,
    anyhow::{anyhow, Result},
    serde::{Serialize, Deserialize},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct EggsSummaryList(pub Vec<EggSummary>);

#[derive(Serialize, Deserialize, Debug)]
pub struct EggSummary {
    pub id: usize,
    pub pid: u32,
    pub name: String,
    pub status: EggStatus,
    pub uptime: String,
    pub retry_count: u32,
}

pub fn summary(_request: &Request, ctx: &Context) -> Result<Response> {
    let state = ctx.state.clone();
    let state = state.lock().map_err(|_| anyhow!("failed to lock state"))?;
    let eggs = state.eggs.clone();
    let mut summary_list = Vec::new();
    
    for (_, egg) in eggs.iter() {
        let summary = EggSummary {
            id: match egg.id {
                Some(ref id) => *id,
                None => 0,
            },
            pid: match egg.state {
                Some(ref state) => state.pid,
                None => 0,
            },
            name: egg.name.clone(),
            status: match egg.state {
                Some(ref state) => state.status.clone(),
                None => EggStatus::Stopped,
            },
            uptime: match egg.state {
                Some(ref state) => {
                    let start_time = state.start_time;
                    if let Some(start_time) = start_time {
                        let now = chrono::Utc::now();
                        humanize_duration(now.signed_duration_since(start_time))
                    } else {
                        "-".to_string()
                    }
                },
                None => "-".to_string(),
            },
            retry_count: match egg.state {
                Some(ref state) => state.try_count,
                None => 0,
            },
        };

        summary_list.push(summary);
    }

    Ok(json(200, EggsSummaryList(summary_list)))
}

pub fn get(request: &Request, ctx: &Context) -> Result<Response> {
    if let Some(id) = request.path_params.get("egg_id") {
        let state = ctx.state.clone();
        let state = state.lock().map_err(|_| anyhow!("failed to lock state"))?;
        if let Some(id) = id.parse::<usize>().ok() {
            if let Some(egg) = state.get(id) {
                return Ok(json(200, egg.clone()));
            }

            return Ok(err(404, format!("egg not found: {}", id)));
        }
    }

    Ok(err(400, "bad request: missing or invalid egg id".to_string()))
}