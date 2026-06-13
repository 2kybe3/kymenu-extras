use std::path::PathBuf;

use clap::{Parser, ValueEnum};

pub static NAME: &str = "kymenu-dir";

#[derive(Debug, Parser)]
#[command(version, name = NAME)]
pub(crate) struct Cli {
    #[arg(required = true, num_args = 1..)]
    pub(crate) paths: Vec<PathBuf>,

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

    #[arg(long)]
    pub(crate) exclude: Vec<String>,

    #[arg(long)]
    pub(crate) hidden: Option<bool>,

    #[arg(long)]
    pub(crate) limit: Option<usize>,

    #[arg(long)]
    pub(crate) ext: Vec<String>,

    #[arg(long)]
    pub(crate) name: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum DisplayMode {
    Absolute,
    Filename,
    Relative,
    RelativePrefixed,
}
