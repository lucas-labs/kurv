pub mod egg;
pub mod state;
pub mod stdio;
pub mod workers;
pub mod spawn;

use {
    crate::common::Info,
    log::error,
    std::process::Command,
    stdio::create_log_file_handles,
    command_group::CommandGroup,
    egg::{Egg, EggStateUpsert},
    stdio::clean_log_handles,
    state::KurvState, 
    workers::Workers
};

/// 🧺 ⇝ encapsulates the main functionality of the server side application
pub struct Kurv {
    pub info: Info,
    pub state: KurvState,
    pub workers: Workers,
}

impl Kurv {
    /// 🧺 ⇝ creates a new instance of the kurv server
    pub fn new() -> Kurv {
        let info = Info::new();
        let kurv_file = info.paths.kurv_file.clone();

        Kurv {
            info,
            state: KurvState::load(kurv_file).unwrap(),
            workers: Workers::new(),
        }
    }

    /// 🧺 ⇝ main loop of the server, it runs twice a second and checks the state 
    /// of the app:
    ///   - if there are any new eggs to spawn (eggs with state `Errored` or `Pending`),
    ///     try to spawn them
    ///   - checks if all the running eggs are still actually running, and if not,
    ///     change their state to `Pending` or `Errored` depending on the reason and
    ///     remove them from the `workers` list so that they can be restarted on the
    ///     next tick
    pub fn run(&mut self) {
        loop {
            // Check if there are any new eggs to spawn
            self.spawn_all();

            // Check if all the running eggs are still actually running
            // self.check_running_eggs();

            // Sleep for a bit
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
