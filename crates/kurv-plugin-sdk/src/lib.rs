//! shared types and entrypoint for kurv plugins.
//!
//! a plugin binary's `main` typically delegates everything to [`start`]:
//!
//! ```no_run
//! use kurv_plugin_sdk::{KurvEnv, PluginConfig, discover_env, plugin_metadata, start};
//!
//! fn main() {
//!     start(
//!         plugin_metadata!(),
//!         |exe| {
//!             let mut env = discover_env(exe).expect("kurv plugin: failed to load sidecar config");
//!             env.insert("MY_PLUGIN_MODE".into(), "dev".into());
//!
//!             PluginConfig {
//!                 name: "my-plugin".into(),
//!                 command: exe.to_string_lossy().into_owned(),
//!                 args: vec!["run".into()],
//!                 env,
//!                 ..Default::default()
//!             }
//!         },
//!         |_env: KurvEnv| {
//!             // plugin loop
//!         },
//!     );
//! }
//! ```

use {
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        env, fmt, fs, io,
        path::{Path, PathBuf},
        process,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PluginMetadata {
    pub name: &'static str,
    pub version: &'static str,
}

impl fmt::Display for PluginMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

#[macro_export]
macro_rules! plugin_metadata {
    () => {
        $crate::PluginMetadata {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        }
    };
}

/// subset of the kurv `Egg` struct that a plugin is allowed to declare via
/// `--kurv-cfg`. the server fills in state / id / plugin flags on its side.
#[derive(Serialize, Default)]
pub struct PluginConfig {
    pub name: String,
    pub command: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
}

#[derive(Deserialize, Default)]
struct SidecarConfig {
    #[serde(default)]
    env: BTreeMap<String, String>,
}

/// discovers an optional `{plugin-name}.config.json` file next to the plugin executable and
/// returns the declared environment variables.
///
/// missing config files are treated as empty configuration. invalid JSON returns an
/// `InvalidData` I/O error so plugins can fail fast with a clear message during `configure`.
pub fn discover_env(exe: &Path) -> io::Result<BTreeMap<String, String>> {
    let config_path = sidecar_config_path(exe);

    let content = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(BTreeMap::new()),
        Err(err) => return Err(err),
    };
    let config = serde_json::from_str::<SidecarConfig>(&content).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to parse plugin sidecar config {}: {}", config_path.display(), err),
        )
    })?;

    Ok(config.env)
}

fn sidecar_config_path(exe: &Path) -> PathBuf {
    let plugin_name = exe.file_stem().and_then(|stem| stem.to_str()).unwrap_or("plugin");
    let parent = exe.parent().unwrap_or_else(|| Path::new("."));
    parent.join(format!("{plugin_name}.config.json"))
}

/// environment variables kurv injects into every plugin process at spawn time.
/// parsed once at `run` dispatch so plugins don't each re-parse them.
pub struct KurvEnv {
    pub api_host: String,
    pub api_port: u16,
    pub home: PathBuf,
    pub logs_dir: PathBuf,
}

impl KurvEnv {
    fn from_env() -> Self {
        Self {
            api_host: env::var("KURV_API_HOST").unwrap_or_default(),
            api_port: env::var("KURV_API_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(0),
            home: env::var_os("KURV_HOME").map(PathBuf::from).unwrap_or_default(),
            logs_dir: env::var_os("KURV_LOGS_DIR").map(PathBuf::from).unwrap_or_default(),
        }
    }
}

/// plugin entrypoint. parses `argv`, then:
///  - `--kurv-cfg`  → calls `configure(&exe)`, prints JSON to stdout, exits 0
///  - `--version|-v` → prints `{plugin-name}@{plugin-version}`, exits 0
///  - `run`         → calls `run_loop(KurvEnv)`, exits 0 on return
///  - anything else → prints usage to stderr, exits 1
pub fn start<C, R>(metadata: PluginMetadata, configure: C, run_loop: R) -> !
where
    C: FnOnce(&Path) -> PluginConfig,
    R: FnOnce(KurvEnv),
{
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("--kurv-cfg") => {
            let exe = env::current_exe().expect("kurv plugin: cannot resolve current exe");
            let cfg = configure(&exe);
            println!("{}", serde_json::to_string(&cfg).expect("kurv plugin: cfg not serializable"));
            process::exit(0);
        }
        Some("--version") | Some("-v") => {
            println!("{}", metadata);
            process::exit(0);
        }
        Some("run") => {
            run_loop(KurvEnv::from_env());
            process::exit(0);
        }
        _ => {
            eprintln!("usage: {} [--kurv-cfg|--version|-v|run]", program_name());
            process::exit(1);
        }
    }
}

fn program_name() -> String {
    env::args()
        .next()
        .as_deref()
        .and_then(|a| Path::new(a).file_name().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "plugin".into())
}
