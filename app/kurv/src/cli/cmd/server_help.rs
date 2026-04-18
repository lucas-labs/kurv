//! # Default command
//! Command executed when no command was specified.
//!
//! This will execute general commands like `help` and `version`, depending
//! on the arguments passed to the program.

use {
    crate::{
        cli::components::{Component, Help},
        printth,
    },
    indoc::indoc,
};

pub fn print() {
    printth!(
        "{}",
        Help {
            command: "kurv server",
            summary: Some(indoc! {"
                starts the <white>kurv</white> server and all its dependant eggs

                <warn><b>!</b></warn> this command will <b>only</b> work if the environment
                variable <white>KURV_SERVER</white> is setted to `true`"
            }),
            error: None,
            options: Some(vec![("--force", vec![], "bypass the KURV_SERVER env var check")]),
            subcommands: None,
        }
        .render()
    );
}
