mod cli;

use std::collections::HashSet;

use clap::Parser;
use common::{InputItem, InputItems};
use walkdir::WalkDir;

use crate::cli::DisplayMode;

fn main() {
    let cli = cli::Cli::parse();
    let mode = cli.mode.unwrap_or_default();
    let hidden = cli.hidden.unwrap_or_default();
    let exts: HashSet<String> = cli
        .ext
        .iter()
        .map(|e| e.trim_start_matches(".").to_owned())
        .collect();

    let file = cli.file.unwrap_or(true);
    let folder = cli.folder.unwrap_or(true);

    let name_regex = cli
        .name
        .as_deref()
        .map(|name| regex::Regex::new(name).unwrap());

    let root = cli.path.canonicalize().unwrap_or(cli.path);

    let mut result: Vec<InputItem> = Vec::new();

    let mut walkdir = WalkDir::new(&root).min_depth(cli.min_depth.unwrap_or(0) + 1);
    if let Some(max_depth) = cli.max_depth {
        walkdir = walkdir.max_depth(max_depth);
    }

    for entry in walkdir
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();

            if !hidden && name.starts_with(".") {
                return false;
            }

            if cli.exclude.iter().any(|x| x == &name) {
                return false;
            }

            true
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !file && path.is_file() {
            continue;
        }
        if !folder && path.is_dir() {
            continue;
        }

        if path.is_file() && !exts.is_empty() {
            let ext = path.extension().and_then(|e| e.to_str());

            if !ext.is_some_and(|e| exts.iter().any(|x| x == e)) {
                continue;
            }
        }

        let name = entry.file_name().to_string_lossy();

        if let Some(ref regex) = name_regex
            && !regex.is_match(&name)
        {
            continue;
        }

        let display = match mode {
            DisplayMode::Filename => name.into_owned(),
            DisplayMode::Absolute => path.display().to_string(),
            DisplayMode::Relative => match path.strip_prefix(&root) {
                Ok(rel) => rel.display().to_string(),
                Err(_) => continue,
            },
        };

        result.push(InputItem::new(display, path.display().to_string()));

        if let Some(limit) = cli.limit
            && result.len() >= limit
        {
            break;
        }
    }

    InputItems::new(result).print()
}
