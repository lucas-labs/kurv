use pico_args::Arguments;

mod api;
pub mod stop_start;
pub mod default;
pub mod list;
pub mod server_help;

/// Returns true if the user wants help with the command
pub fn wants_help(args: &mut Arguments) -> bool {
    args.contains(["-h", "--help"])
}

/// checks if an argument is not an option or a flag (starts with - or --)
pub fn is_option_or_flag(arg: &str) -> bool {
    arg.starts_with('-')
}
