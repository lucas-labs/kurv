use {
    kurv::kurv::Kurv,
    std::{
        env,
        ffi::OsString,
        fs,
        path::Path,
        sync::{Mutex, OnceLock},
    },
    tempfile::TempDir,
};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn test_collect_injects_kurv_env_into_cfg_and_runtime_env() {
    let _lock = env_lock().lock().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("kurv-home");
    let plugins_dir = home_dir.join("plugins");
    let logs_dir = temp_dir.path().join("logs-dir");

    fs::create_dir_all(&plugins_dir).unwrap();
    create_plugin_executable(&plugins_dir, cfg_probe_script());

    let _env = EnvGuard::set([
        ("KURV_HOME", path_env(&home_dir)),
        ("KURV_LOGS_DIR", path_env(&logs_dir)),
        ("KURV_API_HOST", "127.9.9.9".into()),
        ("KURV_API_PORT", "42424".into()),
    ]);

    let (_, state) = Kurv::collect().unwrap();
    let state = state.lock().unwrap();
    let plugins = state.get_plugins();

    assert_eq!(plugins.len(), 1);

    let env = plugins[0].env.as_ref().unwrap();
    assert_eq!(env.get("SEEN_API_HOST"), Some(&"127.9.9.9".to_string()));
    assert_eq!(env.get("SEEN_API_PORT"), Some(&"42424".to_string()));
    assert_eq!(env.get("SEEN_KURV_HOME"), Some(&path_env(&home_dir)));
    assert_eq!(env.get("SEEN_KURV_LOGS_DIR"), Some(&path_env(&logs_dir)));
    assert_eq!(env.get("KURV_API_HOST"), Some(&"127.9.9.9".to_string()));
    assert_eq!(env.get("KURV_API_PORT"), Some(&"42424".to_string()));
    assert_eq!(env.get("KURV_HOME"), Some(&path_env(&home_dir)));
    assert_eq!(env.get("KURV_LOGS_DIR"), Some(&path_env(&logs_dir)));
}

#[test]
fn test_collect_prefers_injected_kurv_env_over_plugin_values() {
    let _lock = env_lock().lock().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("kurv-home");
    let plugins_dir = home_dir.join("plugins");
    let logs_dir = temp_dir.path().join("logs-dir");

    fs::create_dir_all(&plugins_dir).unwrap();
    create_plugin_executable(&plugins_dir, stale_env_script());

    let _env = EnvGuard::set([
        ("KURV_HOME", path_env(&home_dir)),
        ("KURV_LOGS_DIR", path_env(&logs_dir)),
        ("KURV_API_HOST", "127.9.9.9".into()),
        ("KURV_API_PORT", "42424".into()),
    ]);

    let (_, state) = Kurv::collect().unwrap();
    let state = state.lock().unwrap();
    let plugins = state.get_plugins();

    assert_eq!(plugins.len(), 1);

    let env = plugins[0].env.as_ref().unwrap();
    assert_eq!(env.get("KURV_API_HOST"), Some(&"127.9.9.9".to_string()));
    assert_eq!(env.get("KURV_API_PORT"), Some(&"42424".to_string()));
    assert_eq!(env.get("KURV_HOME"), Some(&path_env(&home_dir)));
    assert_eq!(env.get("KURV_LOGS_DIR"), Some(&path_env(&logs_dir)));
}

fn env_lock() -> &'static Mutex<()> {
    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

fn path_env(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

struct EnvGuard {
    previous: Vec<(&'static str, Option<OsString>)>,
}

impl EnvGuard {
    fn set<const N: usize>(vars: [(&'static str, String); N]) -> Self {
        let previous = vars.iter().map(|(key, _)| (*key, env::var_os(key))).collect::<Vec<_>>();

        for (key, value) in vars {
            unsafe {
                env::set_var(key, value);
            }
        }

        Self { previous }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in &self.previous {
            match value {
                Some(value) => unsafe {
                    env::set_var(key, value);
                },
                None => unsafe {
                    env::remove_var(key);
                },
            }
        }
    }
}

#[cfg(windows)]
fn create_plugin_executable(plugins_dir: &Path, script: &str) {
    let script_path = plugins_dir.join("kurv-test.cmd");
    fs::write(script_path, script).unwrap();
}

#[cfg(unix)]
fn create_plugin_executable(plugins_dir: &Path, script: &str) {
    use std::os::unix::fs::PermissionsExt;

    let script_path = plugins_dir.join("kurv-test");
    fs::write(&script_path, script).unwrap();
    let mut permissions = fs::metadata(&script_path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(script_path, permissions).unwrap();
}

#[cfg(windows)]
fn cfg_probe_script() -> &'static str {
    r#"@echo off
if "%1"=="--kurv-cfg" (
  echo {"name":"kurv-test","command":"echo","env":{"SEEN_API_HOST":"%KURV_API_HOST%","SEEN_API_PORT":"%KURV_API_PORT%","SEEN_KURV_HOME":"%KURV_HOME%","SEEN_KURV_LOGS_DIR":"%KURV_LOGS_DIR%"}}
  exit /b 0
)
exit /b 1
"#
}

#[cfg(unix)]
fn cfg_probe_script() -> &'static str {
    r#"#!/bin/sh
if [ "$1" = "--kurv-cfg" ]; then
  printf '%s\n' '{"name":"kurv-test","command":"echo","env":{"SEEN_API_HOST":"'"$KURV_API_HOST"'","SEEN_API_PORT":"'"$KURV_API_PORT"'","SEEN_KURV_HOME":"'"$KURV_HOME"'","SEEN_KURV_LOGS_DIR":"'"$KURV_LOGS_DIR"'"}}'
  exit 0
fi
exit 1
"#
}

#[cfg(windows)]
fn stale_env_script() -> &'static str {
    r#"@echo off
if "%1"=="--kurv-cfg" (
  echo {"name":"kurv-test","command":"echo","env":{"KURV_API_HOST":"stale-host","KURV_API_PORT":"1111","KURV_HOME":"stale-home","KURV_LOGS_DIR":"stale-logs"}}
  exit /b 0
)
exit /b 1
"#
}

#[cfg(unix)]
fn stale_env_script() -> &'static str {
    r#"#!/bin/sh
if [ "$1" = "--kurv-cfg" ]; then
  printf '%s\n' '{"name":"kurv-test","command":"echo","env":{"KURV_API_HOST":"stale-host","KURV_API_PORT":"1111","KURV_HOME":"stale-home","KURV_LOGS_DIR":"stale-logs"}}'
  exit 0
fi
exit 1
"#
}
