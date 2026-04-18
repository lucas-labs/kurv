use {
    crate::{common::Info, kurv::egg::Egg},
    anyhow::{Context, Result, anyhow},
    log::{debug, info, warn},
    std::{
        collections::HashMap,
        path::{Path, PathBuf},
        process::Command,
    },
};

fn injected_plugin_env(info: &Info) -> HashMap<String, String> {
    HashMap::from([
        ("KURV_API_HOST".to_string(), info.api_host.clone()),
        ("KURV_API_PORT".to_string(), info.api_port.to_string()),
        ("KURV_HOME".to_string(), info.paths.kurv_home.display().to_string()),
        ("KURV_LOGS_DIR".to_string(), info.paths.logs_dir.display().to_string()),
    ])
}

fn merge_injected_plugin_env(
    existing_env: Option<HashMap<String, String>>,
    info: &Info,
) -> HashMap<String, String> {
    let mut env = existing_env.unwrap_or_default();
    env.extend(injected_plugin_env(info));
    env
}

/// discovers all plugins in the given directory.
pub fn discover(info: &Info) -> Vec<(PathBuf, Egg)> {
    let dir = &info.paths.plugins_dir;

    if !dir.exists() {
        debug!("plugins directory does not exist at {}", dir.display());
        return vec![];
    }

    debug!("discovering plugins in {}", dir.display());

    // read directory entries
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            warn!("failed to read plugins directory: {}", e);
            return vec![];
        }
    };

    let plugins: Vec<(PathBuf, Egg)> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            // skip directories
            if path.is_dir() {
                return None;
            }

            // check if filename starts with "kurv-"
            let filename = path.file_name()?.to_str()?;
            if !filename.starts_with("kurv-") {
                return None;
            }

            // check if it's executable (on Unix) or has .exe extension (on Windows)
            #[cfg(unix)]
            let is_executable = {
                use std::os::unix::fs::PermissionsExt;
                path.metadata().ok()?.permissions().mode() & 0o111 != 0
            };

            #[cfg(windows)]
            let is_executable = {
                filename.ends_with(".exe")
                    || filename.ends_with(".bat")
                    || filename.ends_with(".cmd")
            };

            if !is_executable {
                return None;
            }

            // try to get plugin configuration
            match get_plugin_config(&path, info) {
                Ok(mut config) => {
                    debug!("successfully loaded plugin config for {}: {:?}", filename, config.name);

                    config.env = Some(merge_injected_plugin_env(config.env.take(), info));

                    Some((path, config))
                }
                Err(e) => {
                    warn!("failed to get config for plugin {}: {}", filename, e);
                    None
                }
            }
        })
        .collect();

    info!("discovered {} plugin(s)", plugins.len());
    plugins
}

/// get plugin configuration
///
/// executes a plugin with `--kurv-cfg` flag, to get its config; parses the JSON output as `Egg`.
fn get_plugin_config(plugin_path: &Path, info: &Info) -> Result<Egg> {
    debug!("getting config for plugin: {}", plugin_path.display());

    let output = Command::new(plugin_path)
        .arg("--kurv-cfg")
        .envs(injected_plugin_env(info))
        .output()
        .with_context(|| format!("failed to execute plugin: {}", plugin_path.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let plugin_name = plugin_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
        return Err(anyhow!("plugin {} exited with non-zero status: {}", plugin_name, stderr));
    }

    // try to get config from stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut config: Egg = serde_json::from_str(&stdout)
        .with_context(|| format!("failed to parse plugin config as JSON: {}", stdout))?;

    // remove id if it has one, to avoid conflicts with existing eggs or plugins; and mark it as
    // a plugin
    config.id = None;
    config.plugin = Some(true);
    config.plugin_path = Some(plugin_path.to_path_buf());
    Ok(config)
}
