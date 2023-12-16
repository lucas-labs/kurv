use anyhow::{anyhow, Result};

use {super::KurvState, crate::kurv::egg::Egg};

impl KurvState {
    /// 🥚 ⇝ adds a new `egg` to the state and **returns** its assigned `id`
    pub fn collect(&mut self, mut egg: Egg) -> usize {
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

    /// 🥚 ⇝ retrieves the egg with the given `id` from the state
    pub fn get(&self, id: usize) -> Option<&Egg> {
        for (_, e) in self.eggs.iter() {
            if e.id == Some(id) {
                return Some(e);
            }
        }

        None
    }

    /// 🥚 ⇝ retrieves the egg with the given `id` from the state as a mutable reference
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Egg> {
        for (_, e) in self.eggs.iter_mut() {
            if e.id == Some(id) {
                return Some(e);
            }
        }

        None
    }

    /// 🥚 ⇝ retrieves the egg with the given `state.pid` from the state
    pub fn get_by_pid(&self, pid: u32) -> Option<&Egg> {
        for (_, e) in self.eggs.iter() {
            if e.state.is_some() && e.state.as_ref().unwrap().pid == pid {
                return Some(e);
            }
        }

        None
    }

    /// 🥚 ⇝ retrieves the egg with the given `state.pid` from the state as a mutable reference
    pub fn get_by_pid_mut(&mut self, pid: u32) -> Option<&mut Egg> {
        for (_, e) in self.eggs.iter_mut() {
            if e.state.is_some() && e.state.as_ref().unwrap().pid == pid {
                return Some(e);
            }
        }

        None
    }

    /// 🥚 ⇝ retrieves the egg with the given `name` from the state
    pub fn get_by_name(&mut self, name: &str) -> Option<&Egg> {
        self.eggs.get(name)
    }

    /// 🥚 ⇝ retrieves the egg with the given `name` from the state as a mutable reference
    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut Egg> {
        self.eggs.get_mut(name)
    }

    /// 🥚 ⇝ removes the egg with the given `name` from the state, and returns it
    ///
    /// **warn:** this will raise an error if the egg is still running. So, make sure to
    /// kill it first.
    pub fn remove(&mut self, name: &str) -> Result<Egg> {
        if let Some(egg) = self.get_by_name(name).cloned() {
            // check that egg.state.pid is None
            if let Some(state) = egg.state.clone() {
                if state.pid > 0 {
                    return Err(anyhow!(
                        "Egg '{}' is still running with pid {}, please stop it first",
                        name,
                        state.pid
                    ));
                }
            }

            self.eggs.remove(name);
            Ok(egg)
        } else {
            Err(anyhow!("Egg with name '{}' not found", name))
        }
    }
}
