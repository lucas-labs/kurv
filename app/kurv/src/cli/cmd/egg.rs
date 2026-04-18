use {
    crate::{
        cli::{
            cmd::{api::Api, is_option_or_flag, wants_help, wants_raw},
            components::{Component, Help},
        },
        common::str::ToString,
        kurv::{Egg, EggStatus},
        printth,
    },
    anyhow::{Result, anyhow},
    indoc::{formatdoc, indoc},
    pico_args::Arguments,
    std::path::PathBuf,
};

/// prints eggs state summary snapshot
pub fn run(args: &mut Arguments) -> Result<()> {
    if wants_help(args) {
        return help();
    }

    let api = Api::new();
    let cmd_arg: Result<Option<String>> =
        args.opt_free_from_str().map_err(|_| anyhow!("wrong usage"));

    if let Ok(Some(id)) = cmd_arg {
        if is_option_or_flag(&id) {
            return Err(anyhow!("wrong usage"));
        }

        let response = api.egg(id.as_str());

        if let Ok(egg) = response {
            if wants_raw(args) {
                printth!("{}", serde_json::to_string_pretty(&egg)?);
                return Ok(());
            }

            let args = match egg.args.clone() {
                Some(args) => args.join(" "),
                None => "".to_string(),
            };

            printth!(
                "{}",
                formatdoc! {
                    "

                    <yellow>⬮</yellow> » <b><white>{}</white></b>
                    
                    <magenta><b>id      </b></magenta>{}
                    <magenta><b>name    </b></magenta>{}
                    <magenta><b>command </b></magenta>{}
                    <magenta><b>cwd     </b></magenta>{}
                    ",
                    egg.name,
                    egg.id.unwrap(),
                    egg.name,
                    egg.command.clone() + " <dim>" + args.as_str() + "</dim>",
                    egg.cwd.clone().unwrap_or(PathBuf::from(".")).display(),
                }
            );

            print_env(&egg);
            println!();
            print_paths(&egg);
            println!();
            print_state(&egg);
        }
    } else {
        help()?;
    }

    Ok(())
}

fn print_state(egg: &Egg) {
    if let Some(state) = &egg.state {
        let status_color = match state.status {
            EggStatus::Pending => "dim",
            EggStatus::Running => "green",
            EggStatus::Stopped => "warn",
            EggStatus::Errored => "error",
            EggStatus::PendingRemoval => "warn",
            EggStatus::Restarting => "magenta",
        };

        let status = state.status.str().to_lowercase();
        let status = status.trim_end();

        printth!(
            "{}",
            formatdoc! {
                "
                <magenta><b>status:    </b></magenta>
                  <white><b>status     </b></white><{}>{}</{}>
                  <white><b>pid        </b></white>{}
                  <white><b>start time </b></white>{}
                  <white><b>try count  </b></white>{}
                  <white><b>error      </b></white>{}
                ",
                status_color,
                status,
                status_color,
                state.pid,
                state.start_time.unwrap_or_default(),
                state.try_count,
                state.error.clone().unwrap_or("".to_string()),
            }
        );
    }
}

fn print_env(egg: &Egg) {
    if let Some(env) = &egg.env {
        printth!("{}", "<magenta><b>env:</b></magenta>");

        let max_key_len = env.keys().map(|k| k.len()).max().unwrap_or(0);

        for (key, value) in env {
            let padding = " ".repeat(max_key_len - key.len());
            printth!("  <white><b>{}</b></white>{} {}", key, padding, value);
        }
    }
}

fn print_paths(egg: &Egg) {
    if let Some(paths) = &egg.paths {
        printth!("{}", "<magenta><b>paths:</b></magenta>");

        let maybe_stdout = paths.stdout.to_str();
        let maybe_stderr = paths.stderr.to_str();

        if let Some(stdout) = maybe_stdout {
            printth!("  <white><b>stdout</b></white> {}", stdout);
        }

        if let Some(stderr) = maybe_stderr {
            printth!("  <white><b>stderr</b></white> {}", stderr);
        }
    }
}

fn help() -> Result<()> {
    printth!(
        "{}",
        Help {
            command: "kurv egg",
            summary: Some(indoc! {
                "shows a snapshot of the current status of a specific egg
                
                <magenta><b>example:</b></magenta>
                  <dim>$</dim> <white><b>kurv</b></white> egg <green>1</green>           <dim># by id</dim>
                  <dim>$</dim> <white><b>kurv</b></white> egg <green>myprocess</green>   <dim># by name</dim>
                  <dim>$</dim> <white><b>kurv</b></white> egg <green>9778</green>        <dim># by pid</dim>" 
            }),
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
