use anyhow::Ok;

use self::cmd::wants_help;

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
            if subcmd == "server" {
                if wants_help(&mut arguments) {
                    cmd::server_help::print();
                    return Ok(DispatchResult::Dispatched);
                }

                // server will be handled by the main function
                Ok(DispatchResult::Server)
            } else {
                // handle other subcommands
                let command_result = match subcmd.as_str() {
                    "list" => cmd::list::run(&mut arguments),
                    _ => cmd::default::run(
                        &mut arguments,
                        Some(
                            format!("Invalid usage | Command '{}' not recognized", subcmd).as_str(),
                        ),
                    ),
                };

                command_result.map(|_| DispatchResult::Dispatched)
            }
        }
        // if there is no subcommand, run the default command
        None => cmd::default::run(&mut arguments, None).map(|_| DispatchResult::Dispatched),
    };

    result.map_err(|err| anyhow!(err))
}
