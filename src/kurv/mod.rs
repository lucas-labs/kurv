use {
    anyhow::Result,
    std::sync::{Arc, Mutex},
};

mod egg;
mod spawn;
mod kill;
mod state;
mod stdio;
mod workers;

pub use egg::EggStatus;
pub use egg::Egg;

use {
    crate::common::Info,
    command_group::CommandGroup,
    egg::EggStateUpsert,
    state::KurvState,
    std::process::Command,
    std::time::Duration,
    std::thread::sleep,
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
            // Check if there are any new eggs to spawn
            self.spawn_all();
            self.check_running_eggs();
            self.check_stopped_eggs();

            // sleep for a bit, we don't want to destroy the cpu
            sleep(Duration::from_millis(500));
        }
    }

    pub fn collect() -> Result<(Arc<Mutex<Info>>, Arc<Mutex<KurvState>>)> {
        let info = Info::new();
        let state = KurvState::load(info.paths.kurv_file.clone()).unwrap();

        Ok((Arc::new(Mutex::new(info)), Arc::new(Mutex::new(state))))
    }
}
