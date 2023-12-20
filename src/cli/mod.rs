use anyhow::Ok;

use self::cmd::{stop_start::StopStartAction, wants_help};

use {
    anyhow::{anyhow, Result},
    pico_args::Arguments,
};

pub mod cmd;
pub mod color;
pub mod components;

pub enum DispatchResult {
    Dispatched,
    Server,
}

pub fn dispatch_command() -> Result<DispatchResult> {
    let mut arguments = Arguments::from_env();
    let subcommand = arguments.subcommand()?;

    let result = match subcommand {
        Some(subcmd) => {
            match subcmd.as_ref() {
                "server" | "s" => {
                    if wants_help(&mut arguments) {
                        cmd::server_help::print();
                        return Ok(DispatchResult::Dispatched);
                    }

                    // server will be handled by the main function
                    Ok(DispatchResult::Server)
                }
                "list" | "l" | "ls" | "snaps" => {
                    cmd::list::run(&mut arguments).map(|_| DispatchResult::Dispatched)
                }
                "stop" => cmd::stop_start::run(&mut arguments, StopStartAction::Stop)
                    .map(|_| DispatchResult::Dispatched),
                "start" => cmd::stop_start::run(&mut arguments, StopStartAction::Start)
                    .map(|_| DispatchResult::Dispatched),
                "remove" => cmd::stop_start::run(&mut arguments, StopStartAction::Remove)
                    .map(|_| DispatchResult::Dispatched),
                "restart" => cmd::stop_start::run(&mut arguments, StopStartAction::Restart)
                    .map(|_| DispatchResult::Dispatched),
                "collect" => cmd::collect::run(&mut arguments).map(|_| DispatchResult::Dispatched),
                _ => cmd::default::run(
                    &mut arguments,
                    Some(format!("Invalid usage | Command '{}' not recognized", subcmd).as_str()),
                )
                .map(|_| DispatchResult::Dispatched),
            }
        }
        // if there is no subcommand, run the default command
        None => cmd::default::run(&mut arguments, None).map(|_| DispatchResult::Dispatched),
    };

    result.map_err(|err| anyhow!(err))
}
