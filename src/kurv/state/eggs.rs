use anyhow::{anyhow, Result};

use {super::KurvState, crate::kurv::egg::Egg};

impl KurvState {
    /// 🥚 » adds a new `egg` to the state and **returns** its assigned `id`
    pub fn collect(&mut self, egg: &Egg) -> usize {
        // from self.eggs find the one with the highest egg.id
        let next_id = self
            .eggs
            .iter()
            .map(|(_, egg)| egg.id.unwrap_or(0))
            .max()
            .unwrap_or(0)
            + 1;

        let mut new_egg = egg.clone();
        new_egg.id = Some(next_id);
        self.eggs.insert(egg.name.clone(), new_egg);

        next_id
    }

    /// 🥚 » retrieves the egg with the given `id` from the state
    pub fn get_by_id(&self, id: usize) -> Option<&Egg> {
        for (_, e) in self.eggs.iter() {
            if e.id == Some(id) {
                return Some(e);
            }
        }

        None
    }

    /// 🥚 » retrieves the egg with the given `id` from the state as a mutable reference
    pub fn get_by_id_mut(&mut self, id: usize) -> Option<&mut Egg> {
        for (_, e) in self.eggs.iter_mut() {
            if e.id == Some(id) {
                return Some(e);
            }
        }

        None
    }

    /// 🥚 » retrieves the egg with the given `state.pid` from the state
    pub fn get_by_pid(&self, pid: u32) -> Option<&Egg> {
        for (_, e) in self.eggs.iter() {
            if e.state.is_some() && e.state.as_ref().unwrap().pid == pid {
                return Some(e);
            }
        }

        None
    }

    /// 🥚 » retrieves the egg with the given `state.pid` from the state as a mutable reference
    pub fn get_by_pid_mut(&mut self, pid: u32) -> Option<&mut Egg> {
        for (_, e) in self.eggs.iter_mut() {
            if e.state.is_some() && e.state.as_ref().unwrap().pid == pid {
                return Some(e);
            }
        }

        None
    }

    /// 🥚 » retrieves the egg with the given `name` from the state
    pub fn get_by_name(&self, name: &str) -> Option<&Egg> {
        self.eggs.get(name)
    }

    /// 🥚 » retrieves the egg with the given `name` from the state as a mutable reference
    pub fn get_by_name_mut(&mut self, name: &str) -> Option<&mut Egg> {
        self.eggs.get_mut(name)
    }

    /// 🥚 » retrieves the `egg` with the given token; the token can be:
    ///   - the internal id of the egg
    ///   - the pid of the running egg
    ///   - the name (key) of the egg
    pub fn get(&self, token: String) -> Option<&Egg> {
        // since we receive a string, we will search by name first to avoid
        // innecesary conversions
        if let Some(egg) = self.get_by_name(token.as_str()) {
            return Some(egg);
        }

        // if we couldn't find it by name, we try by id
        if let Some(id) = token.parse::<usize>().ok() {
            if let Some(egg) = self.get_by_id(id) {
                return Some(egg);
            }
        }

        // and at last, we try by pid
        if let Some(pid) = token.parse::<u32>().ok() {
            if let Some(egg) = self.get_by_pid(pid) {
                return Some(egg);
            }
        }

        // wrong token probably =)
        None
    }

    // 🥚 » returns `true` if there's an agg with name `key`
    pub fn contains_key(&self, key: String) -> bool {
        self.eggs.contains_key(&key)
    }

    /// 🥚 » removes the egg with the given `name` from the state, and returns it
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

    /// 🥚 » stops an egg
    pub fn stop(&mut self, name: &str) -> Result<Egg> {
        if let Some(egg) = self.get_by_name_mut(name) {
            // check that egg.state.pid is None
            egg.set_as_stopped();
            Ok(egg.clone())
        } else {
            Err(anyhow!("Egg with name '{}' not found", name))
        }
    }
}
