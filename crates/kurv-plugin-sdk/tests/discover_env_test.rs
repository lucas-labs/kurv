use {
    kurv_plugin_sdk::discover_env,
    std::{fs, io::ErrorKind},
    tempfile::TempDir,
};

#[test]
fn test_discover_env_reads_sidecar_file() {
    let temp_dir = TempDir::new().unwrap();
    let exe_path = temp_dir.path().join(executable_name());
    let config_path = temp_dir.path().join("kurv-ui.config.json");

    fs::write(&config_path, "{\"env\":{\"MY_ENV_VAR\":\"FOO-BAR\"}}").unwrap();

    let env = discover_env(&exe_path).unwrap();
    assert_eq!(env.get("MY_ENV_VAR"), Some(&"FOO-BAR".to_string()));
}

#[test]
fn test_discover_env_returns_empty_when_sidecar_is_missing() {
    let temp_dir = TempDir::new().unwrap();
    let exe_path = temp_dir.path().join(executable_name());

    let env = discover_env(&exe_path).unwrap();
    assert!(env.is_empty());
}

#[test]
fn test_discover_env_returns_invalid_data_error_for_bad_json() {
    let temp_dir = TempDir::new().unwrap();
    let exe_path = temp_dir.path().join(executable_name());
    let config_path = temp_dir.path().join("kurv-ui.config.json");

    fs::write(&config_path, "{\"env\":").unwrap();

    let err = discover_env(&exe_path).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidData);
}

#[cfg(windows)]
fn executable_name() -> &'static str {
    "kurv-ui.exe"
}

#[cfg(not(windows))]
fn executable_name() -> &'static str {
    "kurv-ui"
}
