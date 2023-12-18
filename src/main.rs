mod api;
mod cli;
mod common;
mod kurv;

use {
    crate::cli::components::{Component, Logo},
    anyhow::Result,
    cli::dispatch_command,
    cli::DispatchResult,
    common::log::Logger,
    kurv::Kurv,
    log::Level,
    std::thread,
};

fn main() -> Result<()> {
    Logger::init(Level::Trace)?;

    let result = match dispatch_command()? {
        DispatchResult::Dispatched => Ok(()),
        DispatchResult::Server => {
            printth!("{}", (Logo {}).render());
            let (info, state) = Kurv::collect()?;

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
