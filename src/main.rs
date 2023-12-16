mod api;
mod cli;
mod common;
mod kurv;

use {
    anyhow::Result,
    cli::dispatch_command,
    cli::DispatchResult,
    common::Info,
    kurv::{state::KurvState, Kurv},
    std::{
        sync::{Arc, Mutex},
        thread,
    },
};

fn main() -> Result<()> {
    let result = match dispatch_command()? {
        DispatchResult::Dispatched => Ok(()),
        DispatchResult::Server => {
            let (info, state) = get_info_and_state()?;

            // start the api server on its own thread
            let api_info = info.clone();
            let api_state = state.clone();
            thread::spawn(move || {
                api::start(api_info, api_state);
            });

            // 🏃 run forest, run!
            Kurv::new(info.clone(), state.clone()).run();
            Ok(())
        }
    };

    result
}

fn get_info_and_state() -> Result<(Arc<Mutex<Info>>, Arc<Mutex<KurvState>>)> {
    let info = Info::new();
    let state = KurvState::load(info.paths.kurv_file.clone()).unwrap();

    Ok((Arc::new(Mutex::new(info)), Arc::new(Mutex::new(state))))
}
