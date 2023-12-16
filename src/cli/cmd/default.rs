//! # Default command
//! Command executed when no command was specified.
//!
//! This will execute general commands like `help` and `version`, depending
//! on the arguments passed to the program.

use {
    crate::cli::components::{Component, Help},
    crate::printth,
    anyhow::Result,
    pico_args::Arguments,
};

pub fn run(args: &mut Arguments, err: Option<&str>) -> Result<()> {
    if args.contains(["-v", "--version"]) {
        print_version();
        return Ok(());
    }

    // by default, print help
    help(err);
    Ok(())
}

fn help(err: Option<&str>) {
    printth!(
        "{}",
        Help {
            command: "kurv",
            summary: Some("Just a simple process manager =)"),
            error: err,
            options: Some(vec![
                ("-h, --help", "Prints help information"),
                ("-v, --version", "Prints version information"),
            ]),
            subcommands: Some(vec![
                ("server", "Starts the kurv server"),
                ("list", "Lists all running processes"),
            ]),
        }
        .render()
    );
}

fn print_version() {
    let version = env!("CARGO_PKG_VERSION").to_string();
    printth!("<dim>kurv@</dim><white>v{version}</white>");

    // TODO: in the future we could show local version and remote version
}
