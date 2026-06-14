use clap::{ArgAction, Parser};

pub static NAME: &str = "kymenu-ssh";

#[derive(Debug, Parser)]
#[command(version, about, name = NAME)]
pub(crate) struct Cli {
    #[arg(
        long,
        num_args = 0..=1,
        require_equals = true,
        default_value_t = true,
        action = ArgAction::Set,
        help = "Show usernames in output",
    )]
    pub(crate) username: bool,

    #[arg(
        long,
        num_args = 0..=1,
        require_equals = true,
        default_value_t = true,
        action = ArgAction::Set,
        help = "Show hostnames in output",
    )]
    pub(crate) hostname: bool,

    #[arg(
        long,
        num_args = 0..=1,
        require_equals = true,
        default_value_t = false,
        action = ArgAction::Set,
        help = "Include system SSH config (/etc/ssh/ssh_config)",
    )]
    pub(crate) system_config: bool,

    #[arg(
        long,
        num_args = 0..=1,
        require_equals = true,
        default_value_t = true,
        action = ArgAction::Set,
        help = "Include user SSH config (~/.ssh/config)",
    )]
    pub(crate) user_config: bool,

    #[arg(
        long,
        value_name = "REGEX",
        help = "Only include hosts whose display name matches the regex"
    )]
    pub(crate) name_rgx: Option<String>,

    #[arg(
        long,
        value_name = "REGEX",
        help = "Only include hosts whose username matches the regex"
    )]
    pub(crate) user_rgx: Option<String>,

    #[arg(
        long,
        value_name = "REGEX",
        help = "Only include hosts whose hostname matches the regex"
    )]
    pub(crate) host_rgx: Option<String>
}
