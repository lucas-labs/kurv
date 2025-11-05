use {
    super::KurvState,
    crate::kurv::egg::Egg,
    anyhow::{Result, anyhow},
};

impl KurvState {
    /// 🥚 » adds a new `egg` to the state and **returns** its assigned `id`
    pub fn collect(&mut self, egg: &Egg) -> usize {
        // from self.eggs find the one with the highest egg.id
        let next_id = self.eggs.values().map(|egg| egg.id.unwrap_or(0)).max().unwrap_or(0) + 1;

        let mut new_egg = egg.clone();
        new_egg.id = Some(next_id);
        self.eggs.insert(egg.name.clone(), new_egg);

        next_id
    }

    /// 🥚 » retrieves the egg with the given `id` from the state
    pub fn get(&self, id: usize) -> Option<&Egg> {
        self.eggs.values().find(|&e| e.id == Some(id))
    }

    /// 🥚 » retrieves the egg with the given `id` from the state as a mutable reference
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Egg> {
        self.eggs.iter_mut().map(|(_, e)| e).find(|e| e.id == Some(id))
    }

    /// 🥚 » retrieves the egg with the given `name` from the state
    pub fn get_by_name(&self, name: &str) -> Option<&Egg> {
        self.eggs.get(name)
    }

    /// 🥚 » retrieves the egg with the given `pid` from the state
    pub fn get_by_pid(&self, pid: u32) -> Option<&Egg> {
        self.eggs.values().find(|&e| e.state.is_some() && e.state.as_ref().unwrap().pid == pid)
    }

    // 🥚 » returns `true` if there's an agg with name `key`
    pub fn contains_key(&self, key: String) -> bool {
        self.eggs.contains_key(&key)
    }

    /// 🥚 » retrieves the `egg.id` with the given token; the token can be:
    ///   - the id of the egg (as a string)
    ///   - the pid of the running egg
    ///   - the name (key) of the egg
    pub fn get_id_by_token(&self, token: &str) -> Option<usize> {
        // Try to parse the token as usize to check if it's an id
        if let Ok(id) = token.parse::<usize>() {
            if let Some(egg) = self.get(id) {
                return egg.id;
            }
        }

        // Try to find an egg with the given pid and return its id
        if let Ok(pid) = token.parse::<u32>() {
            if let Some(egg) = self.get_by_pid(pid) {
                return egg.id;
            }
        }

        // Check if the token corresponds to an egg name and return its id
        if let Some(egg) = self.get_by_name(token) {
            return egg.id;
        }

        // If no match found, return None
        None
    }

    /// 🥚 » removes the egg with the given `name` from the state, and returns it
    ///
    /// **warn:** this will raise an error if the egg is still running. So, make sure to
    /// kill it first.
    pub fn remove(&mut self, id: usize) -> Result<Egg> {
        if let Some(egg) = self.get(id).cloned() {
            // check that egg.state.pid is None
            if let Some(state) = egg.state.clone() {
                if state.pid > 0 {
                    return Err(anyhow!(
                        "Egg '{}' is still running with pid {}, please stop it first",
                        egg.name,
                        state.pid
                    ));
                }
            }

            self.eggs.remove(&egg.name);
            Ok(egg)
        } else {
            Err(anyhow!("Egg with id '{}' not found", id))
        }
    }
}
