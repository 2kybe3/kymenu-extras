mod cli;

use clap::Parser;
use common::{InputItem, InputItems};
use walkdir::WalkDir;

use crate::cli::DisplayMode;

fn main() {
    let cli = cli::Cli::parse();
    let mode = cli.mode.unwrap_or_default();

    let file = cli.file.unwrap_or(true);
    let folder = cli.folder.unwrap_or(true);

    let root = cli.path.canonicalize().unwrap_or(cli.path);

    let mut result: Vec<InputItem> = Vec::new();

    let mut walkdir = WalkDir::new(&root).min_depth(cli.min_depth.unwrap_or(0) + 1);
    if let Some(max_depth) = cli.max_depth {
        walkdir = walkdir.max_depth(max_depth);
    }

    for entry in walkdir.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !file && path.is_file() {
            continue;
        }
        if !folder && path.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy();

        let display = match mode {
            DisplayMode::Filename => name.into_owned(),
            DisplayMode::Absolute => path.display().to_string(),
            DisplayMode::Relative => match path.strip_prefix(&root) {
                Ok(rel) => rel.display().to_string(),
                Err(_) => continue,
            },
        };

        result.push(InputItem::new(display, path.display().to_string()));
    }

    InputItems::new(result).print()
}
