use {
    anyhow::Result,
    env::{current_dir, current_exe},
    serde::{Deserialize, Serialize},
    std::{env, path::PathBuf},
};

const KURV_HOME_KEY: &str = "KURV_HOME";

/// Important paths for the application
#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Paths {
    /// the path of the executable
    pub executable: PathBuf,

    /// the working directory of the application, might be different
    /// from the executable path
    pub working_dir: PathBuf,

    /// the path to the kurv home directory; it will be the parent dir of the executable
    /// or the value of the KURV_HOME environment variable if it is set
    pub kurv_home: PathBuf,

    /// the path to the .kurv file in the kurv home directory
    pub kurv_file: PathBuf,
}

/// General information about the app
#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Info {
    /// the process id of the application
    pub pid: u32,

    /// the version of the application
    pub version: String,

    /// important paths for the application
    pub paths: Paths,
}

impl Info {
    /// Creates a new instance of Info
    pub fn new() -> Info {
        Info {
            pid: std::process::id(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            paths: Info::get_paths().expect("could not get paths"),
        }
    }

    /// Gets the paths for the application
    fn get_paths() -> Result<Paths> {
        let executable = current_exe().expect("could not get executable path");
        let working_dir = current_dir().expect("could not get working directory");

        // kurv home is the parent dir of the executable or the
        // value of the KURV_HOME environment variable if it is set
        let kurv_home = match env::var(KURV_HOME_KEY) {
            Ok(home) => PathBuf::from(home),
            Err(_) => executable.parent().unwrap().to_path_buf(),
        };

        // the path to the .kurv file in the kurv
        // home directory it might not exist yet
        let kurv_file = kurv_home.join(".kurv");

        Ok(Paths {
            executable,
            working_dir,
            kurv_home,
            kurv_file,
        })
    }
}
