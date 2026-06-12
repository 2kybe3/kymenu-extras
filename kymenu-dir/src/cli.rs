use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version)]
pub(crate) struct Cli {
    pub(crate) path: PathBuf,

    #[arg(short, long)]
    pub(crate) mode: Option<DisplayMode>,

    #[arg(long)]
    pub(crate) max_depth: Option<usize>,

    #[arg(long)]
    pub(crate) min_depth: Option<usize>,

    #[arg(long)]
    pub(crate) file: Option<bool>,

    #[arg(long)]
    pub(crate) folder: Option<bool>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum DisplayMode {
    Absolute,
    Filename,
    #[default]
    Relative,
}
