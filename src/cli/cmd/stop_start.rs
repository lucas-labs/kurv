use anyhow::anyhow;
use indoc::formatdoc;

use crate::cli::cmd::wants_raw;

use {
    crate::cli::{
        cmd::{api::Api, is_option_or_flag, wants_help},
        components::{Component, Help},
    },
    crate::printth,
    anyhow::Result,
    indoc::indoc,
    pico_args::Arguments,
};

/// indicates wether we want to stop or start an egg
pub enum StopStartAction {
    Stop,
    Start,
    Remove,
    Restart,
}

struct Strings<'a> {
    action: &'a str,
    doing_action: &'a str,
    past_action: &'a str,
}

/// stops a runnig egg
///
/// IDEA: it works asynchronously, this means that wehen the command
/// ends, the egg might still be running. We could implement a --timeout X
/// option that will check the actual status of the egg until it IS actually
/// Stopped (has no pid), or reaches timeouts (in which case it should end
/// with an error exit code)
pub fn run(args: &mut Arguments, action: StopStartAction) -> Result<()> {
    let strings = get_strings(action);

    if wants_help(args) {
        return help(strings);
    }

    let api = Api::new();
    let cmd_arg: Result<Option<String>> =
        args.opt_free_from_str().map_err(|_| anyhow!("wrong usage"));

    match cmd_arg {
        Ok(maybe_arg) => match maybe_arg {
            Some(id) => {
                if is_option_or_flag(&id) {
                    return Err(anyhow!("wrong usage"));
                }

                let json_resp = wants_raw(args);

                if !json_resp {
                    printth!(
                        "\n<yellow>⬮</yellow> <dim>{} egg {}</dim>\n",
                        strings.doing_action,
                        id
                    );
                }

                let response = api.eggs_post(format!("/{}/{}", id, strings.action).as_str(), "");

                if let Ok(egg) = response {
                    if json_resp {
                        printth!("{}", serde_json::to_string_pretty(&egg)?);
                        return Ok(());
                    }

                    printth!(
                        indoc! {
                            "egg <green>{}</green> has been scheduled to be {}
                             
                            <head><b>i</b></head> you can check its status by running:
                              <dim>$</dim> <white><b>kurv</b></white> egg <green>1</green>
                            "
                        },
                        egg.name,
                        strings.past_action
                    );
                }

                Ok(())
            }
            None => help(strings),
        },
        Err(e) => Err(e),
    }
}

fn help(strings: Strings) -> Result<()> {
    printth!(
        "{}",
        Help {
            command: format!("kurv {}", strings.action).as_ref(),
            summary: Some(formatdoc! {
                "schedules an egg to be {} by the kurv server
                
                <head><b>example:</b></head>
                    <dim>$</dim> <white><b>kurv</b></white> {} <green>1</green>           <dim># by id</dim>
                    <dim>$</dim> <white><b>kurv</b></white> {} <green>myprocess</green>   <dim># by name</dim>
                    <dim>$</dim> <white><b>kurv</b></white> {} <green>9778</green>        <dim># by pid</dim>",
                strings.past_action,
                strings.action,
                strings.action,
                strings.action,
            }.as_ref()),
            error: None,
            options: Some(vec![
                ("-h, --help", vec![], "Prints this help message"),
                ("-j, --json", vec![], "Prints the response in json format")
            ]),
            subcommands: None
        }
        .render()
    );

    Ok(())
}

fn get_strings<'a>(action: StopStartAction) -> Strings<'a> {
    match action {
        StopStartAction::Start => Strings {
            action: "start",
            doing_action: "starting",
            past_action: "started",
        },
        StopStartAction::Stop => Strings {
            action: "stop",
            doing_action: "stopping",
            past_action: "stopped",
        },
        StopStartAction::Remove => Strings {
            action: "remove",
            doing_action: "removing",
            past_action: "removed",
        },
        StopStartAction::Restart => Strings {
            action: "restart",
            doing_action: "restarting",
            past_action: "restarted",
        },
    }
}
