use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum};

pub static NAME: &str = "kymenu-dir";

#[derive(Debug, Parser)]
#[command(version, about, name = NAME)]
pub(crate) struct Cli {
    #[arg(num_args = 1.., help = "List of paths to process")]
    pub(crate) paths: Vec<PathBuf>,

    #[arg(
        short,
        long,
        help = "Specifies how the result should be output for kyvim to display to you"
    )]
    pub(crate) mode: Option<DisplayMode>,

    #[arg(long, help = "Maximum traversal depth")]
    pub(crate) max_depth: Option<usize>,

    #[arg(long, help = "Minimum traversal depth")]
    pub(crate) min_depth: Option<usize>,

    #[arg(
        long,
        action = ArgAction::Set,
        default_value_t = true,
        help = "Specifies whether to include files"
    )]
    pub(crate) file: bool,

    #[arg(
        long,
        action = ArgAction::Set,
        default_value_t = true,
        help = "Specifies whether to include folders"
    )]
    pub(crate) folder: bool,

    #[arg(long, help = "List of paths to not process")]
    pub(crate) exclude: Vec<String>,

    #[arg(
        long,
        action = ArgAction::Set,
        default_value_t = false,
        help = "Specifies whether to process hidden files"
    )]
    pub(crate) hidden: bool,

    #[arg(long, help = "Specifies the limit of files to process before exiting")]
    pub(crate) limit: Option<usize>,

    #[arg(long, help = "Specifies which file extensions to include")]
    pub(crate) ext: Vec<String>,

    #[arg(
        long,
        help = "Specifies a regex that must match the filename to be included"
    )]
    pub(crate) file_rgx: Option<String>,

    #[arg(
        long,
        help = "Specifies a regex that must match the path name to be included"
    )]
    pub(crate) path_rgx: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum DisplayMode {
    #[value(help = "Show absolute paths")]
    Absolute,
    #[value(help = "Show only filenames")]
    Filename,
    #[value(help = "Show paths relative to the root folder")]
    Relative,
    #[value(help = "Show paths relative to the root folder with the root folder prefixed")]
    RelativePrefixed,
}
