pub mod load;

use {
    chrono::prelude::*,
    chrono::Duration,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, path::PathBuf},
};

/// defines the status of an egg
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum EggStatus {
    Pending,
    Running,
    Stopped,
    PendingRemoval,
    Errored,
}

/// defines the watch section of an egg
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Watch {
    on: Vec<String>,
    except: Vec<String>,
}

fn default_pid() -> u32 {
    0
}

/// defines the current state of an egg
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct EggState {
    pub status: EggStatus,
    pub start_time: Option<DateTime<Local>>,
    pub try_count: u32,
    pub error: Option<String>,

    #[serde(default = "default_pid")]
    pub pid: u32,
}

/// partial EggState used as a temporal struct to update the final EggState
pub struct EggStateUpsert {
    pub status: Option<EggStatus>,
    pub start_time: Option<DateTime<Local>>,
    pub try_count: Option<u32>,
    pub error: Option<String>,
    pub pid: Option<u32>,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct EggPaths {
    pub stdout: PathBuf,
    pub stderr: PathBuf,
}

/// 🥚 ⇝ an egg represents a process that can be started and stopped by kurv
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Egg {
    pub command: String,
    pub name: String,

    /// unique id of the egg
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<EggState>,

    /// files to watch for changes that will trigger a restart
    /// TODO: not yet implemented
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<Watch>,

    /// arguments to be passed to the command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// working directory at which the command will be run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<PathBuf>,

    /// environment variables to be set before running the command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    /// paths to the stdout and stderr log files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<EggPaths>,
}

impl Egg {
    /// checks that the `egg` has a `state` or
    /// creates a new one if it doesn't.
    fn validate_state(&mut self) {
        if self.state.is_none() {
            self.state = Some(EggState {
                status: EggStatus::Pending,
                start_time: None,
                try_count: 0,
                error: None,
                pid: 0,
            });
        }
    }

    /// if `self` already has a `state`, it will be updated,
    /// otherwise a new `EggState` will be created for the `egg`.
    ///
    /// `EggStateUpsert` is a temporal struct
    /// that allows to update **only** the fields that are not `None`.
    pub fn upsert_state(&mut self, state: EggStateUpsert) {
        if let Some(ref mut egg_state) = self.state {
            if let Some(status) = state.status {
                egg_state.status = status;
            }
            if let Some(start_time) = state.start_time {
                egg_state.start_time = Some(start_time);
            }
            if let Some(try_count) = state.try_count {
                egg_state.try_count = try_count;
            }
            if let Some(error) = state.error {
                egg_state.error = Some(error);
            }
            if let Some(pid) = state.pid {
                egg_state.pid = pid;
            }
        } else {
            self.state = Some(EggState {
                status: state.status.unwrap_or(EggStatus::Pending),
                start_time: state.start_time,
                try_count: state.try_count.unwrap_or(0),
                error: state.error,
                pid: state.pid.unwrap_or(0),
            });
        }
    }

    /// sets the `status` of the `egg` to the given `status`.
    pub fn set_status(&mut self, status: EggStatus) {
        self.validate_state();

        if let Some(ref mut egg_state) = self.state {
            egg_state.status = status;
        }
    }

    /// increments the `try_count` of the `egg` by 1.
    pub fn increment_try_count(&mut self) {
        self.validate_state();

        if let Some(ref mut egg_state) = self.state {
            egg_state.try_count += 1;
        }
    }

    /// resets the `try_count` of the `egg` to 0.
    pub fn reset_try_count(&mut self) {
        self.validate_state();

        if let Some(ref mut egg_state) = self.state {
            egg_state.try_count = 0;
        }
    }

    /// sets the `pid` of the `egg` to the given `pid`.
    pub fn set_pid(&mut self, pid: u32) {
        self.validate_state();

        // set the pid if the egg has a state
        if let Some(ref mut egg_state) = self.state {
            egg_state.pid = pid;
        }
    }

    // sets the `error` of the `egg` to the given `error`.
    pub fn set_error(&mut self, error: String) {
        self.validate_state();

        // set the error if the egg has a state
        if let Some(ref mut egg_state) = self.state {
            egg_state.error = Some(error);
        }
    }

    /// sets the `start_time` of the `egg` to the current time.
    pub fn reset_start_time(&mut self) {
        self.validate_state();

        // set the start time if the egg has a state
        if let Some(ref mut egg_state) = self.state {
            egg_state.start_time = Some(Local::now());
        }
    }

    /// sets the `start_time` of the `egg`
    pub fn set_start_time(&mut self, time: Option<DateTime<Local>>) {
        self.validate_state();

        // set the start time if the egg has a state
        if let Some(ref mut egg_state) = self.state {
            egg_state.start_time = time;
        }
    }

    /// marks the `egg` as running by:
    /// - setting the `pid` of the `egg` to the given `pid`.
    /// - setting the `start_time` of the `egg` to the current time.
    /// - resetting the `try_count` of the `egg` to 0.
    /// - setting the `status` of the `egg` to `EggStatus::Running`.
    pub fn set_as_running(&mut self, pid: u32) {
        self.set_pid(pid);
        self.reset_start_time();
        self.set_status(EggStatus::Running);
        self.set_error("".to_string());
    }

    /// marks the `egg` as errored by:
    pub fn set_as_errored(&mut self, error: String) {
        self.set_error(error);
        self.set_status(EggStatus::Errored);
        self.set_pid(0);
        self.increment_try_count();
    }

    /// marks the `egg` as stopped by:
    pub fn set_as_stopped(&mut self) {
        if !self.is_pending_removal() {
            self.set_status(EggStatus::Stopped);
        }

        self.set_pid(0);
        self.reset_try_count();
        self.set_start_time(None);
    }

    /// checks if the `egg` should be spawned
    /// (if its state is `Pending` or `Errored`).
    ///
    /// if it doesn't have a state, it should be spawned, as it's probably
    /// a new egg that has just been added.
    pub fn should_spawn(&self) -> bool {
        if let Some(ref egg_state) = self.state {
            egg_state.status == EggStatus::Pending || egg_state.status == EggStatus::Errored
        } else {
            true
        }
    }

    pub fn has_been_running_for(&self, duration: Duration) -> bool {
        if let Some(ref egg_state) = self.state {
            if let Some(start_time) = egg_state.start_time {
                let now = Local::now();
                let diff = now.signed_duration_since(start_time);
                diff > duration
            } else {
                false
            }
        } else {
            false
        }
    }

    /// checks if the `egg` is running
    /// (if its state is `Running`).
    pub fn is_running(&self) -> bool {
        if let Some(ref egg_state) = self.state {
            egg_state.status == EggStatus::Running
        } else {
            false
        }
    }

    /// checks if the `egg` is stopped
    /// (if its state is `Stopped`).
    pub fn is_stopped(&self) -> bool {
        if let Some(ref egg_state) = self.state {
            egg_state.status == EggStatus::Stopped
        } else {
            false
        }
    }

    /// checks if the `egg` is pending removal
    /// (if its state is `PendingRemoval`).
    pub fn is_pending_removal(&self) -> bool {
        if let Some(ref egg_state) = self.state {
            egg_state.status == EggStatus::PendingRemoval
        } else {
            false
        }
    }
}
