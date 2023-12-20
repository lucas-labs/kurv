//! # Default command
//! Command executed when no command was specified.
//!
//! This will execute general commands like `help` and `version`, depending
//! on the arguments passed to the program.

use indoc::indoc;

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
            summary: Some(indoc!{
                "just a simple process manager =)
                
                <white><b>!</b></white> you can also use <white>kurv [command] --help</white> to get 
                help information about a specific command.
                
                <dim><white><b>»</b></white> example</dim>
                
                <dim>$</dim> <white>kurv</white> server <dim>--help</dim>"
            }),
            error: err,
            options: Some(vec![
                ("-h, --help", vec![], "prints help information"),
                ("-v, --version", vec![], "prints version information"),
            ]),
            subcommands: Some(vec![
                ("server", vec!["s"], "starts the kurv server"),
                ("list", vec!["l"], "prints eggs list and their statuses"),
                ("stop", vec![], "stops a running egg"),
                ("start", vec![], "starts a stopped egg"),
                ("remove", vec![], "removes an egg"),
                ("collect", vec![], "collects and starts a new egg"),
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
