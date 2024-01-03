mod egg;
mod kill;
mod spawn;
mod state;
mod stdio;
mod workers;

pub use egg::Egg;
pub use egg::EggState;
pub use egg::EggStatus;

use {
    crate::common::Info,
    anyhow::Result,
    command_group::CommandGroup,
    egg::EggStateUpsert,
    state::KurvState,
    std::process::Command,
    std::sync::{Arc, Mutex},
    std::thread::sleep,
    std::time::Duration,
    stdio::clean_log_handles,
    stdio::create_log_file_handles,
    workers::Workers,
};

pub type KurvStateMtx = Arc<Mutex<KurvState>>;
pub type InfoMtx = Arc<Mutex<Info>>;

/// encapsulates the main functionality of the server side application
pub struct Kurv {
    pub info: InfoMtx,
    pub state: KurvStateMtx,
    pub workers: Workers,
}

impl Kurv {
    /// creates a new instance of the kurv server
    pub fn new(info: InfoMtx, state: KurvStateMtx) -> Kurv {
        Kurv {
            info,
            state,
            workers: Workers::new(),
        }
    }

    /// main loop of the server, it runs twice a second and checks the state
    /// of the app:
    ///   - if there are any new eggs to spawn (eggs with state `Errored` or `Pending`),
    ///     try to spawn them
    ///   - checks if all the running eggs are still actually running, and if not,
    ///     change their state to `Pending` or `Errored` depending on the reason and
    ///     remove them from the `workers` list so that they can be re-started on the
    ///     next tick
    ///   - check if all eggs that were marked as stopped are actually stopped and
    ///     kill them otherwise
    pub fn run(&mut self) {
        loop {
            // each check returns an "unsynced" flag that tell us wether the state
            // has changed and we need to sync state with its file system file.
            // this avoids unnecesary write operations
            let mut unsynced = false;

            unsynced = self.spawn_all() || unsynced;
            unsynced = self.check_running_eggs() || unsynced;
            unsynced = self.check_stopped_eggs() || unsynced;

            // removal needs to happen after stops, to avoid orphans
            unsynced = self.check_removal_pending_eggs() || unsynced;

            if unsynced {
                // let state = self.state.clone();
                let state = self.state.lock().unwrap();
                let info = self.info.lock().unwrap();
                state.save(&info.paths.kurv_file).unwrap();
            }

            // sleep for a bit, we don't want to destroy the cpu
            sleep(Duration::from_millis(500));
        }
    }

    /// loads application state from .kurv file.
    ///
    /// this should only be called on bootstrap, as it will expect all
    /// eggs to not be running
    pub fn collect() -> Result<(InfoMtx, KurvStateMtx)> {
        let info = Info::new();
        let mut state = KurvState::load(info.paths.kurv_file.clone()).unwrap();

        // replace running eggs to Pending status, so they are started
        // on bootstra
        for (_, egg) in state.eggs.iter_mut() {
            if let Some(ref mut state) = egg.state {
                if state.status == EggStatus::Running {
                    state.status = EggStatus::Pending;
                }
            }
        }

        Ok((Arc::new(Mutex::new(info)), Arc::new(Mutex::new(state))))
    }
}
