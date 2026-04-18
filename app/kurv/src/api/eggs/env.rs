use {
    super::{Context, err},
    crate::{
        api::eggs::{NOT_FOUND_MSG, WRONG_ID_MSG},
        common::tcp::{Request, Response, json},
    },
    anyhow::{Result, anyhow},
    std::collections::HashMap,
};

/// merge an egg's environment variable configuration with existing env vars
pub fn merge(request: &Request, ctx: &Context) -> Result<Response> {
    update_env(request, ctx, false)
}

/// replace an egg's environment variable configuration
pub fn replace(request: &Request, ctx: &Context) -> Result<Response> {
    update_env(request, ctx, true)
}

pub fn update_env(request: &Request, ctx: &Context, replace: bool) -> Result<Response> {
    if let Some(token) = request.path_params.get("egg_id") {
        let state = ctx.state.clone();
        let mut state = state.lock().map_err(|_| anyhow!("failed to lock state"))?;
        let id = state.get_id_by_token(token);

        if let Some(id) = id
            && let Some(egg) = state.get_mut(id)
        {
            let env: HashMap<String, String> = serde_json::from_str(&request.body)?;

            if replace {
                egg.env = Some(env);
            } else {
                let existing_env = egg.env.clone().unwrap_or_default();
                let merged_env = existing_env.into_iter().chain(env).collect();
                egg.env = Some(merged_env);
            }

            egg.set_synced(false);

            return Ok(json(200, egg.clone()));
        }

        return Ok(err(404, format!("{}: {}", NOT_FOUND_MSG, token)));
    }

    Ok(err(400, WRONG_ID_MSG.to_string()))
}
