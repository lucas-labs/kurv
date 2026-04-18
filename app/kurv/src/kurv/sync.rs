use super::Kurv;

impl Kurv {
    /// checks each egg looking for those that have pending unsynced states. This usually means that
    /// the state of the egg was changed (e.g. environment variables updated); different from sync
    /// caused by status changes (e.g. from Running to Stopped)
    pub(crate) fn check_unsynced_eggs(&mut self) -> bool {
        let state = self.state.clone();
        let mut state = state.lock().unwrap();
        let mut unsynced: bool = false;

        for (_, egg) in state.eggs.iter_mut() {
            // if the egg is not stopped or pending removal, continue

            if !egg.is_state_unsynced() {
                continue;
            }

            // we have an egg with unsynced state, we need to trigger a disk save
            // and set the egg as synced so that next loop it won't be detected again
            egg.set_synced(true);

            // the main loop will see that we have unsynced changes and will save to disk
            unsynced = true;
        }

        unsynced
    }
}
