use {
    super::{egg::EggPaths, *},
    chrono::Duration,
    command_group::GroupChild,
    log::{debug, error, warn},
};

impl Kurv {
    /// try to spawn all eggs that are in `Pending` or `Errored` state
    pub(crate) fn spawn_all(&mut self) -> bool {
        let state = self.state.clone();
        let mut state = state.lock().unwrap();
        let mut unsynced = false;

        let mut eggs = state.eggs.clone();
        for (key, egg) in eggs.iter_mut() {
            // if the egg is errored or pending, try to spawn it
            if egg.should_spawn() {
                let (updated_egg, child) = self.spawn_egg(egg);

                // update original egg in state.eggs with the new values
                state.eggs.insert(key.clone(), updated_egg);

                // if child is Some, add it to the workers
                if let Some(child) = child {
                    // so, we have a running egg, let's add it to the worker
                    self.workers
                        .add_child(None, key.clone(), egg.id.unwrap(), child);
                }

                unsynced = true;
            }
        }

        unsynced
    }

    /// checks each eggs looking for those that have finished running unexpectedly
    /// and sets their state accordingly. Also keeps re-try count updated
    pub(crate) fn check_running_eggs(&mut self) -> bool {
        let state = self.state.clone();
        let mut state = state.lock().unwrap();
        let mut unsynced: bool = false;

        for (_, egg) in state.eggs.iter_mut() {
            // if the egg is not running, then it was probably already checked
            if !egg.is_running() {
                continue;
            }

            // if the egg doesn't have an id, it means it hasn't been spawned yet
            let id = match egg.id {
                Some(id) => id,
                None => {
                    continue;
                }
            };

            if let Some(child) = self.workers.get_child_mut(id) {
                // check that the child is still running
                match child.inner().try_wait() {
                    Ok(None) => {
                        // if it has been running for more than 5 seconds, we can assume
                        // it started correctly and reset the try count just in case
                        if egg.has_been_running_for(Duration::seconds(5)) {
                            egg.reset_try_count();
                        }
                    }
                    Ok(Some(status)) => {
                        // yikes, the egg has exited, let's update its state
                        let exit_err_msg: String = match status.code() {
                            Some(code) => format!("Exited with code {}", code),
                            None => "Exited with unknown code".to_string(),
                        };

                        // try to get the try count from the egg
                        let try_count = match &egg.state {
                            Some(state) => state.try_count,
                            None => 0,
                        };

                        warn!(
                            "egg <green>{}</green> exited: {} [#{}]",
                            egg.name, exit_err_msg, try_count
                        );

                        egg.set_as_errored(exit_err_msg);
                        unsynced = true
                    }
                    Err(e) => {
                        error!("error while waiting for child process {}: {}", id, e);
                        continue;
                    }
                }
            }
        }

        unsynced
    }

    /// spawns the given `egg` and adds it to the `workers` list
    fn spawn_egg(&mut self, egg: &Egg) -> (Egg, Option<GroupChild>) {
        let info = &self.info.lock().unwrap();
        let mut egg = egg.clone();
        let egg_name = egg.name.clone();
        let log_dir = info.paths.kurv_home.clone();

        let ((stdout_path, stdout_log), (stderr_path, stderr_log)) =
            match create_log_file_handles(&egg_name, &log_dir) {
                Ok((stdout_log, stderr_log)) => (stdout_log, stderr_log),
                Err(err) => {
                    panic!("failed to create log file handles: {}", err)
                }
            };

        egg.paths = Some(EggPaths {
            stdout: stdout_path,
            stderr: stderr_path,
        });

        let (command, cwd, args, envs) = {
            (
                egg.command.clone(),
                match egg.cwd.clone() {
                    Some(cwd) => cwd,
                    None => info.paths.working_dir.clone(),
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
            .envs(envs.unwrap_or_else(std::collections::HashMap::new))
            .group_spawn();

        // check if it has been spawned correctly
        let child = match process {
            Ok(child) => child,
            Err(err) => {
                let error = format!("failed to spawn child {egg_name} with err: {err:?}");
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

        debug!("spawned egg <green>{}</green>", egg.name);

        (egg, Some(child))
    }
}
