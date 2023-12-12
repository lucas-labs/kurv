use command_group::GroupChild;

use super::*;

impl Kurv {
    /// 🧺 ⇝ for each egg in the `eggs` collection, tries to spawn it
    /// if it's state is `Errored` or `Pending`
    pub fn spawn_all(&mut self) {
        let mut eggs = self.state.eggs.clone();
        for (key, egg) in eggs.iter_mut() {
            if egg.should_spawn() {
                let (updated_egg, child) = self.spawn_egg(&egg);

                // update original egg in self.state.eggs with the new values
                self.state.eggs.insert(key.clone(), updated_egg);

                // if child is Some, add it to the workers
                if let Some(child) = child {
                    // so, we have a running egg, let's add it to the worker and update its state
                    // set_as_running will update the egg's pid, state, start_time and try_count
                    // egg.set_as_running(child.id());
                    self.workers
                        .add_child(None, key.clone(), egg.id.unwrap(), child);
                }
            }
        }
    }

    /// 🧺 ⇝ spawns the given `egg` and adds it to the `workers` list
    fn spawn_egg(&mut self, egg: &Egg) -> (Egg, Option<GroupChild>) {
        let mut egg = egg.clone();

        let egg_name = egg.name.clone();
        let log_dir = &self.info.paths.kurv_home.clone();

        let (stdout_log, stderr_log) = match create_log_file_handles(&egg_name, &log_dir) {
            Ok((stdout_log, stderr_log)) => (stdout_log, stderr_log),
            Err(err) => {
                panic!("Failed to create log file handles: {}", err)
            }
        };

        let (command, cwd, args, envs) = {
            (
                egg.command.clone(),
                match egg.cwd.clone() {
                    Some(cwd) => cwd,
                    None => self.info.paths.working_dir.clone(),
                },
                egg.args.clone(),
                egg.env.clone(),
            )
        };

        // Chain the args method call directly to the Command creation and configuration
        let process = Command::new(command)
            .current_dir(cwd)
            .stdout(stdout_log)
            .stderr(stderr_log)
            .args(args.unwrap_or_else(Vec::new))
            .envs(envs.unwrap_or_else(|| std::collections::HashMap::new()))
            .group_spawn();

        // check if it has been spawned correctly
        let child = match process {
            Ok(child) => child,
            Err(err) => {
                let error = format!("Failed to spawn child {egg_name} with err: {err:?}");
                error!("{}", error);
                clean_log_handles(&egg_name, &log_dir);

                // Update all necessary fields on the task.
                egg.upsert_state(EggStateUpsert {
                    status: Some(egg::EggStatus::Errored),
                    error: Some(error),
                    pid: Some(0),
                    start_time: None,
                    try_count: None,
                });

                // Increment the try count
                egg.increment_try_count();

                // Return the updated egg
                return (egg, None);
            }
        };

        egg.set_as_running(child.id());

        (egg, Some(child))
    }
}
