use pico_args::Arguments;

pub mod api;
pub mod default;
pub mod list;
pub mod server_help;

/// Returns true if the user wants help with the command
pub fn wants_help(args: &mut Arguments) -> bool {
    args.contains(["-h", "--help"])
}
