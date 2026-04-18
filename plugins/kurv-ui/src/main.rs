use {
    kurv_plugin_sdk::{KurvEnv, PluginConfig, start},
    std::{env, thread, time::Duration},
};

fn main() {
    start(
        |exe| PluginConfig {
            name: "kurv-ui".into(),
            command: exe.to_string_lossy().into_owned(),
            args: vec!["run".into()],
            env: [("HELLO_MESSAGE".into(), "Hello from kurv-ui plugin!".into())].into(),
            ..Default::default()
        },
        run,
    );
}

fn run(env: KurvEnv) {
    println!("HELLO_MESSAGE: {}", env::var("HELLO_MESSAGE").unwrap_or_default());
    println!("KURV_API_HOST: {}", env.api_host);
    println!("KURV_API_PORT: {}", env.api_port);
    println!("KURV_HOME:     {}", env.home.display());
    println!("KURV_LOGS_DIR: {}", env.logs_dir.display());

    loop {
        thread::sleep(Duration::from_secs(5));
    }
}
