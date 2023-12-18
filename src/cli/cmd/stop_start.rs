use anyhow::anyhow;
use indoc::formatdoc;

use {
    crate::cli::{
        cmd::{api::Api, wants_help, is_option_or_flag},
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
    Start
}

struct Strings<'a> {
    action: &'a str,
    doing_action: &'a str,
    past_action: &'a str,
}


/// stops a runnig egg
/// 
/// IDEA: it works asynchronously, this means that ehen the command 
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
    let cmd_arg: Result<Option<String>> = args.opt_free_from_str().map_err(|_| anyhow!("wrong usage"));

    match cmd_arg {
        Ok(maybe_arg) => {
            match maybe_arg {
                Some(id) => {
                    if is_option_or_flag(&id) {
                        return Err(anyhow!("wrong usage"));
                    }

                    printth!("\n<white>🥚</white> <dim>{} egg {}</dim>\n", strings.doing_action, id);

                    let response = api.stop_start_egg(id, strings.action.to_string());

                    match response {
                        Ok(egg) => {
                            printth!(indoc! {
                                "egg <green>{}</green> has been scheduled to be {}
                                 
                                <head><b>i</b></head> you can check its status by running:
                                  <dim>$</dim> <white><b>kurv</b></white> stat <green>1</green>
                                "
                            }, strings.past_action, egg.name);
                        },
                        _ => {}
                    }

                    Ok(())
                }
                None => Ok(())
            }
        }
        Err(e) => Err(e)
    }
}

fn help(strings: Strings) -> Result<()> {
    printth!(
        "{}",
        Help {
            command: "kurv stop",
            summary: Some(formatdoc! {
                "schedules an egg to be {} by the kurv server
                
                <head><b>example:</b></head>
                  <dim>-> if we want to {} egg #<green>1</green> (you can use <white>kurv list</white>
                  to check ids):</dim>
                  
                  <dim>$</dim> <white><b>kurv</b></white> {} <green>1</green>",
                strings.past_action,
                strings.action,
                strings.action,
            }.as_ref()),
            error: None,
            options: Some(vec![
                ("-h, --help", vec![], "Prints this help message"),
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
            past_action: "started"
        },
        StopStartAction::Stop => Strings {
            action: "stop",
            doing_action: "stopping",
            past_action: "stopped"
        }
    }
}