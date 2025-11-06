pub mod eggs;

use {
    super::egg::Egg,
    crate::common::str::ToString,
    anyhow::{Context, Result},
    log::debug,
    serde::{Deserialize, Serialize},
    std::{collections::BTreeMap, fs::File, path::PathBuf},
};

/// KurvState encapsulates the state of the server side application
/// It's serialized to disk as a YAML file and loaded on startup
#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct KurvState {
    pub eggs: BTreeMap<String, Egg>,
}

impl KurvState {
    /// tries to load the state from the given
    /// path, or creates a new one if it doesn't exist
    pub fn load(path: &PathBuf) -> Result<KurvState> {
        if !path.exists() {
            debug!(".kurv file not found, starting fresh (searched in {})", path.display());
            debug!("you can set KURV_HOME to change the directory");
            return Ok(KurvState {
                eggs: BTreeMap::new(),
            });
        }

        let rdr = File::open(&path)
            .with_context(|| format!("failed to open eggs file: {}", path.display()))?;

        // try to deserialize as JSON first, fall back to YAML for backward compatibility
        // TODO: DEPRECATE -> remove YAML support in future versions
        let mut state: KurvState = match serde_json::from_reader(&rdr) {
            Ok(state) => {
                debug!("loaded state from JSON format");
                state
            }
            Err(json_err) => {
                debug!("failed to parse as JSON, trying YAML format: {}", json_err);
                // Reopen the file since the reader was consumed
                let rdr = File::open(&path)
                    .with_context(|| format!("failed to reopen eggs file: {}", path.display()))?;

                KurvState::deserialize(serde_saphyr::from_reader(rdr)).with_context(|| {
                    format!("failed to parse eggs file as JSON or YAML: {}", path.display())
                })?
            }
        };

        // remove all existing plugins from state to start fresh
        state.eggs.retain(|_, egg| !egg.plugin.unwrap_or(false));

        // reassign ids to all eggs
        let mut next_id = 1;
        for (_, egg) in state.eggs.iter_mut() {
            egg.id = Some(next_id);
            next_id += 1;
        }

        debug!("{} eggs collected!", state.eggs.len());
        Ok(state)
    }

    /// saves the state to the given path
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, serialized)?;

        let trim: &[_] = &['\r', '\n'];
        debug!("saving state to {}", path.str().trim_matches(trim));

        Ok(())
    }
}
