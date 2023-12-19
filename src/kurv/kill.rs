use {
    super::Kurv,
    log::{debug, error},
};

impl Kurv {
    /// checks each egg looking for those that are still running but that were
    /// marked as stopped from the api. In case it finds such a case, then it
    /// kills the background process of the egg that's supposed to be stopped.
    pub(crate) fn check_stopped_eggs(&mut self) -> bool {
        let state = self.state.clone();
        let mut state = state.lock().unwrap();
        let mut unsynced: bool = false;

        for (_, egg) in state.eggs.iter_mut() {
            // if the egg is not stopped or pending removal, continue

            let is_pending_removal = egg.is_pending_removal();
            let is_stopped = egg.is_stopped();

            if !is_stopped && !is_pending_removal {
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
                        unsynced = true;

                        debug!("egg <green>{}</green> has been stopped", egg.name);
                    }
                    Ok(_) => {
                        // it's stopped, but we still have it in the workers for some
                        // odd reason (shouldn't happen)... well, let's remove it.
                        self.workers.remove_child(None, egg.name.clone());
                        // just in case...
                        egg.set_as_stopped();
                        unsynced = true;

                        debug!("egg <green>{}</green> is stopped but was still on the workers list, it has now been removed", egg.name);
                    }
                    Err(e) => {
                        error!("error while waiting for child process {}: {}", id, e);
                        continue;
                    }
                }
            } else {
                // there's not child yet, it might've started as Stopped or PendingRemoval
                // let's clean status to show that there nothing running
                // - set_as_stopped will change status to Stopped only if current status is
                //   not PendingRemoval. This will allow the removal to take place.
                egg.set_as_stopped();
            }
        }

        unsynced
    }

    /// checks each egg looking for those that has its removal pending
    /// and removes them from the state.
    pub(crate) fn check_removal_pending_eggs(&mut self) -> bool {
        let state = self.state.clone();
        let mut state = state.lock().unwrap();
        let mut unsynced: bool = false;

        let eggs = state.eggs.clone();

        for (_, egg) in eggs {
            // if the is not pending removal, continue
            if !egg.is_pending_removal() {
                continue;
            }

            let _ = state.remove(&egg.name);

            debug!("egg <green>{}</green> has been removed", egg.name);
            unsynced = true
        }

        unsynced
    }
}
