use {
    serde::{Deserialize, Serialize},
    strum_macros::{Display, EnumString},
};

/// This is a small custom Enum for all currently supported unix signals.
/// Supporting all unix signals would be a mess, since there is a LOT of them.
///
/// This is also needed for usage in clap, since nix's Signal doesn't implement [Display] and
/// [std::str::FromStr].
#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize, Display, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Signal {
    #[strum(serialize = "sigint", serialize = "int", serialize = "2")]
    SigInt,
    #[strum(serialize = "sigkill", serialize = "kill", serialize = "9")]
    SigKill,
    #[strum(serialize = "sigterm", serialize = "term", serialize = "15")]
    SigTerm,
    #[strum(serialize = "sigcont", serialize = "cont", serialize = "18")]
    SigCont,
    #[strum(serialize = "sigstop", serialize = "stop", serialize = "19")]
    SigStop,
}

