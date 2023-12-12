use {
    anyhow::Result,
    serde::{Deserialize, Serialize},
    std::{collections::BTreeMap, fs::File, path::PathBuf},
};

use anyhow::Context;
use indoc::formatdoc;

use crate::kurv::Egg;

/// KurvState encapsulates the state of the server side application
/// It's serialized to disk as a JSON file and loaded on startup
#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct KurvState {
    pub eggs: BTreeMap<String, Egg>,
}

impl KurvState {
    /// 🥚 ⇝ adds a new `egg` to the state and **returns** its assigned `id`
    pub fn add_egg(&mut self, mut egg: Egg) -> usize {
        // from self.eggs find the one with the highest egg.id
        let next_id = self
            .eggs
            .iter()
            .map(|(_, egg)| egg.id.unwrap_or(0))
            .max()
            .unwrap_or(0)
            + 1;
        
        egg.id = Some(next_id);
        self.eggs.insert(egg.name.clone(), egg);

        next_id
    }


    /// 🥚 ⇝ tries to load the state from the given 
    /// path, or creates a new one if it doesn't exist
    pub fn load(path: PathBuf) -> Result<KurvState> {
        if !path.exists() {
            return Ok(KurvState {
                eggs: BTreeMap::new(),
            });
        }

        let rdr = File::open(&path)
            .with_context(|| formatdoc!("Failed to open eggs file: {}", path.display()))?;

        let mut state: KurvState = serde_yaml::from_reader(rdr)
            .context(format!("Failed to parse eggs file: {}", path.display()))?;

        // check that all the eggs have an id and if not, assign one
        let mut next_id = 1;
        for (_, egg) in state.eggs.iter_mut() {
            if egg.id.is_none() {
                egg.id = Some(next_id);
                next_id += 1;
            } else {
                next_id = egg.id.unwrap() + 1;
            }
        }

        Ok(state)
    }

    /// 🥚 ⇝ saves the state to the given path
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let serialized = serde_yaml::to_string(&self)?;
        std::fs::write(path, serialized)?;

        Ok(())
    }
}
