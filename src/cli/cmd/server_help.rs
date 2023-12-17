//! # Default command
//! Command executed when no command was specified.
//!
//! This will execute general commands like `help` and `version`, depending
//! on the arguments passed to the program.

use indoc::indoc;

use {
    crate::cli::components::{Component, Help},
    crate::printth
};


pub fn print() {
    printth!(
        "{}",
        Help {
            command: "kurv server",
            summary: Some(indoc!{"
                starts the <white>kurv</white> server and all its dependant eggs

                <warn><b>!</b></warn> this command will <b>only</b> work if the environment
                variable <white>KURV_SERVER</white> is setted to `true`"
            }),
            error: None,
            options: None,
            subcommands: None,
        }
        .render()
    );
}
