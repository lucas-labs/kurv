use {
    crate::kurv::Egg,
    anyhow::anyhow,
    indoc::formatdoc,
    std::{path::PathBuf, process::exit},
};

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

/// collects a new egg
pub fn run(args: &mut Arguments) -> Result<()> {
    if wants_help(args) {
        return help();
    }

    let api = Api::new();
    let cmd_arg: Result<Option<String>> =
        args.opt_free_from_str().map_err(|_| anyhow!("wrong usage"));

    match cmd_arg {
        Ok(maybe_arg) => {
            if let Some(path) = maybe_arg {
                if is_option_or_flag(&path) {
                    return Err(anyhow!("wrong usage"));
                }

                printth!("\n🥚 <dim>collecting new egg</dim>\n");

                match Egg::load(PathBuf::from(path)) {
                    Ok(egg) => {
                        let body = serde_json::to_string(&egg).unwrap();

                        // call the api
                        let response = api.eggs_post("", body.as_ref());

                        // check response
                        if let Ok(egg) = response {
                            printth!(
                                "{}",
                                formatdoc! {
                                    "egg <green>{}</green> has been collected with id {} and
                                    scheduled to be started
                                        
                                    <head><b>i</b></head> you can check its status by running:
                                        <dim>$</dim> <white><b>kurv</b></white> egg <green>{}</green>
                                    ",
                                    egg.name,
                                    egg.id.unwrap_or(0),
                                    egg.id.unwrap_or(0),
                                }
                            );
                        }
                    }
                    Err(_) => exit(1),
                }

                Ok(())
            } else {
                help()
            }
        }
        Err(e) => Err(e),
    }
}

fn help() -> Result<()> {
    printth!(
        "{}",
        Help {
            command: "kurv stop",
            summary: Some(
                indoc! {
                    "collects an egg and schedules it to be started.
                
                <head><b>example:</b></head>
                  <dim>-> if we want to collect the egg <green>./egg.kurv</green>:</dim>
                  
                  <dim>$</dim> <white><b>kurv</b></white> collect <green>./egg.kurv</green>",
                }
            ),
            error: None,
            options: Some(vec![("-h, --help", vec![], "Prints this help message"),]),
            subcommands: None
        }
        .render()
    );

    Ok(())
}
