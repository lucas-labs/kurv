use {
    super::Kurv,
    log::{debug, error},
};

impl Kurv {
    /// checks each egg looking for those that are still running but that were
    /// marked as stopped from the api. In case it finds such a case, then it
    /// kills the background process of the egg that's supposed to be stopped.
    pub(crate) fn check_stopped_eggs(&mut self) {
        let state = self.state.clone();
        let mut state = state.lock().unwrap();

        for (_, egg) in state.eggs.iter_mut() {
            // if the egg is not stopped, continue
            if !egg.is_stopped() {
                continue;
            }

            // if the egg doesn't have an id, it means it hasn't been spawned yet
            // so, we won't need to stop anything, continue.
            let id = match egg.id {
                Some(id) => id,
                None => {
                    continue;
                }
            };

            if let Some(child) = self.workers.get_child_mut(id) {
                // check if the egg is actually running when it shouldn't
                match child.inner().try_wait() {
                    Ok(None) => {
                        // it's still running, let's kill the mf
                        // kill errors when there's nothing to kill, in this case,
                        // we can ignore the error.
                        let _ = child.kill();

                        // TODO: we can't ingore it actually xD
                        // match child.kill() {
                        //     Ok(_) => Ok(()),
                        //     Err(ref e) if e.kind() == std::io::ErrorKind::InvalidData => {
                        //         // Process already exited
                        //         info!("Task {task_id} has already finished by itself.");
                        //         Ok(())
                        //     }
                        //     Err(err) => Err(err),
                        // }

                        // we should also remove the child from the workers map and
                        // set the egg as stopped (clear its pid, etc, not just the state)
                        self.workers.remove_child(None, egg.name.clone());
                        egg.set_as_stopped();

                        debug!("egg <green>{}</green> has been stopped", egg.name);
                    }
                    Ok(_) => {
                        // it's stopped, but we still have it in the workers for some
                        // odd reason (shouldn't happen)... well, let's remove it.
                        self.workers.remove_child(None, egg.name.clone());
                        // just in case...
                        egg.set_as_stopped();

                        debug!("egg <green>{}</green> is stopped but was still on the workers list, it has now been removed", egg.name);
                    }
                    Err(e) => {
                        error!("error while waiting for child process {}: {}", id, e);
                        continue;
                    }
                }
            }
        }
    }
}
