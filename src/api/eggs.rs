use {
    super::{Context, err},
    crate::{
        common::{
            duration::humanize_duration,
            str::ToString,
            tcp::{Request, Response, json},
        },
        kurv::{Egg, EggState, EggStatus},
    },
    anyhow::{Result, anyhow},
    serde::{Deserialize, Serialize},
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

const WRONG_ID_MSG: &str = "missing or invalid egg id";
const NOT_FOUND_MSG: &str = "egg not found";
const CANNOT_REMOVE_MSG: &str = "plugins cannot be removed via API";

pub fn summary(request: &Request, ctx: &Context) -> Result<Response> {
    let state = ctx.state.clone();
    let state = state.lock().map_err(|_| anyhow!("failed to lock state"))?;
    let kind = request.query_params.get("kind").map(|s| s.as_str()).unwrap_or("eggs");

    let eggs = match kind {
        "plugins" => state.get_plugins(),
        "eggs" => state.get_eggs(),
        _ => state.get_eggs(),
    };

    // let eggs = state.get_eggs(include_plugins);
    let mut summary_list = Vec::new();

    for egg in eggs {
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
                Some(ref state) => state.status,
                None => EggStatus::Pending,
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
                }
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
    if let Some(token) = request.path_params.get("egg_id") {
        let state = ctx.state.clone();
        let state = state.lock().map_err(|_| anyhow!("failed to lock state"))?;

        let id = state.get_id_by_token(token);

        if let Some(id) = id
            && let Some(egg) = state.get(id)
        {
            return Ok(json(200, egg.clone()));
        }

        return Ok(err(404, format!("{}: {}", NOT_FOUND_MSG, token)));
    }

    Ok(err(400, WRONG_ID_MSG.to_string()))
}

/// stop a running egg
pub fn stop(request: &Request, ctx: &Context) -> Result<Response> {
    set_status(request, ctx, EggStatus::Stopped)
}

/// start a running egg
pub fn start(request: &Request, ctx: &Context) -> Result<Response> {
    set_status(request, ctx, EggStatus::Pending)
}

/// remove an egg
pub fn remove(request: &Request, ctx: &Context) -> Result<Response> {
    set_status(request, ctx, EggStatus::PendingRemoval)
}

/// restart a running egg
pub fn restart(request: &Request, ctx: &Context) -> Result<Response> {
    set_status(request, ctx, EggStatus::Restarting)
}

/// changes the status of an egg
pub fn set_status(request: &Request, ctx: &Context, status: EggStatus) -> Result<Response> {
    if let Some(token) = request.path_params.get("egg_id") {
        let state = ctx.state.clone();
        let mut state = state.lock().map_err(|_| anyhow!("failed to lock state"))?;

        let id = state.get_id_by_token(token);

        if let Some(id) = id
            && let Some(egg) = state.get_mut(id)
        {
            match status {
                EggStatus::Pending => {
                    // we can only change to pending if its state is currently Stopped
                    if let Some(state) = egg.state.clone()
                        && state.status != EggStatus::Stopped
                    {
                        return Ok(err(400, format!("egg {} is already running", egg.name)));
                    }
                }
                EggStatus::Stopped => {}
                EggStatus::PendingRemoval => {
                    // prevent removing plugins via this endpoint
                    if egg.is_plugin() {
                        return Ok(err(403, CANNOT_REMOVE_MSG.to_string()));
                    }
                }
                EggStatus::Restarting => {}
                _ => {
                    let trim: &[_] = &['\r', '\n'];
                    return Ok(err(
                        400,
                        format!("can't change status to '{}'", status.str().trim_matches(trim)),
                    ));
                }
            };

            egg.set_status(status);
            return Ok(json(200, egg.clone()));
        }

        return Ok(err(404, format!("{}: {}", NOT_FOUND_MSG, token)));
    }

    Ok(err(400, WRONG_ID_MSG.to_string()))
}

/// changes the status of an egg to Stopped or Pending
pub fn collect(request: &Request, ctx: &Context) -> Result<Response> {
    let maybe_egg: Result<Egg, _> = serde_json::from_str(&request.body);

    match maybe_egg {
        Ok(mut egg) => {
            let state = ctx.state.clone();
            let mut state = state.lock().map_err(|_| anyhow!("failed to lock state"))?;

            if state.contains_key(egg.name.clone()) {
                return Ok(err(
                    409,
                    format!("An egg with name {} already exists", egg.name.clone()),
                ));
            }

            // set egg state as pendig
            let egg_state = match egg.state.clone() {
                Some(state) => {
                    let mut new_state = state.clone();
                    new_state.status = EggStatus::Pending;

                    new_state
                }
                None => EggState {
                    status: EggStatus::Pending,
                    start_time: None,
                    try_count: 0,
                    error: None,
                    pid: 0,
                },
            };

            egg.state = Some(egg_state);
            let id = state.collect(&egg);
            egg.id = Some(id);

            Ok(json(200, egg))
        }
        Err(error) => Ok(err(400, format!("Invalid egg: {}", error))),
    }
}
