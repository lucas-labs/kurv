use {
    crate::{
        cli::{
            cmd::{api::Api, is_option_or_flag, wants_help, wants_raw},
            components::{Component, Help},
        },
        kurv::Egg,
        printth,
    },
    anyhow::{Result, anyhow},
    indoc::{formatdoc, indoc},
    pico_args::Arguments,
    std::{collections::HashMap, fs::File, path::PathBuf},
};

/// manage environment variables for eggs
pub fn run(args: &mut Arguments) -> Result<()> {
    if wants_help(args) {
        return help();
    }

    let api = Api::new();
    let replace = should_replace(args);
    let cmd_id: Result<Option<String>> =
        args.opt_free_from_str().map_err(|_| anyhow!("wrong usage"));
    let cmd_path: Result<Option<String>> =
        args.opt_free_from_str().map_err(|_| anyhow!("wrong usage"));

    if let (Ok(Some(id)), Ok(Some(path))) = (cmd_id, cmd_path) {
        if is_option_or_flag(&id) || is_option_or_flag(&path) {
            return Err(anyhow!("wrong usage"));
        }

        // read the file as json and parse it as a hashmap str,str for validation
        let env_file = get_env_from_file(&path)?;
        let response = api.update_egg_env(&id, &env_file, replace);

        if let Ok(egg) = response {
            if wants_raw(args) {
                printth!("{}", serde_json::to_string_pretty(&egg)?);
                return Ok(());
            }

            printth!(
                "{}",
                formatdoc! {
                    "

                    <yellow>⬮</yellow> » <b><white>{}</white></b>
                    
                    <green>Successfully updated environment variables for egg '{}'!</green>
                    ",
                    egg.name,
                    egg.name,
                }
            );

            print_env(&egg);

            printth!(
                "{}",
                formatdoc! {
                    "
                    
                    <orange>Changes will take effect the next time the egg is (re)started.</orange>
                    "
                }
            );
            return Ok(());
        }

        Ok(())
    } else {
        Err(anyhow!("wrong usage"))
    }
}

fn get_env_from_file(path: &str) -> Result<HashMap<String, String>> {
    let path_buf = PathBuf::from(path);
    let rdr = File::open(&path_buf)
        .map_err(|_| anyhow!("failed to open file: {}", path_buf.display()))?;
    let env_file: HashMap<String, String> = serde_json::from_reader(rdr)
        .map_err(|_| anyhow!("failed to parse json from file: {}", path_buf.display()))?;

    Ok(env_file)
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

fn help() -> Result<()> {
    printth!(
        "{}",
        Help {
            command: "kurv env",
            summary: Some(indoc! {
                "manages environment variables for eggs
                
                <magenta><b>example:</b></magenta>
                  <dim>$</dim> <white><b>kurv</b></white> env <green>1</green> my-env.json          <dim># by id</dim>
                  <dim>$</dim> <white><b>kurv</b></white> env <green>myprocess</green> my-env.json  <dim># by name</dim>
                  <dim>$</dim> <white><b>kurv</b></white> env <green>9778</green> my-env.json       <dim># by pid</dim>" 
            }),
            error: None,
            options: Some(vec![
                ("-h, --help", vec![], "Prints this help message"), 
                ("-j, --json", vec![], "Prints the response in json format"),
                ("--replace", vec![], "Replaces the entire environment variables with the provided ones")
            ]),
            subcommands: None
        }
        .render()
    );

    Ok(())
}

pub fn should_replace(args: &mut Arguments) -> bool {
    args.contains("--replace")
}
