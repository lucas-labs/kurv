mod api;
mod cli;
mod common;
mod kurv;

use {
    crate::cli::components::{Component, Logo},
    anyhow::Result,
    cli::{DispatchResult, dispatch_command},
    common::log::Logger,
    indoc::formatdoc,
    kurv::Kurv,
    log::Level,
    pico_args::Arguments,
    std::{process::exit, thread},
};

fn main() -> Result<()> {
    Logger::init(Level::Trace)?;

    match dispatch_command()? {
        DispatchResult::Dispatched => Ok(()),
        DispatchResult::Server => {
            if !can_run_as_server() {
                exit(1);
            }

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
    }
}

/// check if the app can run as a server
///
/// the app can run as a server if the KURV_SERVER env var is set to true
fn can_run_as_server() -> bool {
    // check that the KURV_SERVER env var is set to true
    let mut arguments = Arguments::from_env();
    if arguments.contains("--force") {
        return true;
    }

    match std::env::var("KURV_SERVER") {
        Ok(val) => val == "true",
        Err(_) => {
            printth!(
                "{}",
                formatdoc! {"

                <error>[error]</error> to be able to run kurv as a server, the <white>KURV_SERVER</white> env var must be 
                set to <white>true</white>.

                <b><head>why though?</head></b> 
                since <white>kurv</white> cli can run both as a server and as a client using the same 
                executable, it might be the case that you want be sure that you won't
                accidentally launch the server when you meant to launch the client.

                So, to be able to run it as a server, you must explicitly set the 
                <white>KURV_SERVER</white> environment variable to <white>true</white> clearly indicate your intention.

                You can bypass this check by sending the <white>--force</white> flag to the cli.
            "}
            );

            false
        }
    }
}
