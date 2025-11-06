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

    /// the path to the plugins directory inside the kurv home directory
    pub plugins_dir: PathBuf,

    /// logs directory
    pub logs_dir: PathBuf,
}

/// General information about the app
#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct Info {
    /// the version of the application
    pub name: String,

    /// the version of the application
    pub version: String,

    /// API Hostname
    pub api_host: String,

    /// API Port
    pub api_port: u16,

    /// the version compilation name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<&'static str>,

    /// description
    pub description: String,

    /// the process id of the application
    pub pid: u32,

    /// important paths for the application
    pub paths: Paths,
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl Info {
    /// Creates a new instance of Info
    pub fn new() -> Info {
        Info {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            api_host: std::env::var("KURV_API_HOST").unwrap_or("127.0.0.1".to_string()),
            api_port: std::env::var("KURV_API_PORT")
                .unwrap_or("58787".to_string())
                .parse::<u16>()
                .unwrap_or(58787),
            version_name: option_env!("KURV_VERSION_NAME"),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
            pid: std::process::id(),
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

        // logs directory, try to get it from KURV_LOGS_DIR env variable
        // or default to kurv_home/task_logs
        let logs_dir = match env::var("KURV_LOGS_DIR") {
            Ok(dir) => PathBuf::from(dir),
            Err(_) => kurv_home.join("task_logs"),
        };

        // the path to the .kurv file in the kurv home directory (it might not exist yet)
        let kurv_file = kurv_home.join(".kurv");

        // the path to the plugins directory inside the kurv home directory
        let plugins_dir = kurv_home.join("plugins");

        Ok(Paths {
            executable,
            working_dir,
            kurv_home,
            kurv_file,
            plugins_dir,
            logs_dir,
        })
    }
}
