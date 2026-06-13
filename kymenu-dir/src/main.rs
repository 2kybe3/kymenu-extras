mod cli;
mod extracted;

use std::path::PathBuf;

use clap::Parser;
use common::{InputItem, InputItems};
use walkdir::WalkDir;

use crate::{
    cli::{Cli, DisplayMode},
    extracted::Extracted,
};

fn extract_results(results: &mut Vec<InputItem>, extracted: &Extracted, base_path: &PathBuf) {
    let mut walkdir = WalkDir::new(base_path).min_depth(extracted.min_depth);
    if let Some(max_depth) = extracted.max_depth {
        walkdir = walkdir.max_depth(max_depth);
    }

    for entry in walkdir
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();

            if !extracted.hidden && name.starts_with(".") {
                return false;
            }

            if extracted.exclude.iter().any(|x| x == &name) {
                return false;
            }

            true
        })
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        if !extracted.file && entry_path.is_file() {
            continue;
        }
        if !extracted.folder && entry_path.is_dir() {
            continue;
        }

        if entry_path.is_file() && !extracted.ext.is_empty() {
            let ext = entry_path.extension().and_then(|e| e.to_str());

            if !ext.is_some_and(|e| extracted.ext.iter().any(|x| x == e)) {
                continue;
            }
        }

        let name = entry.file_name().to_string_lossy();

        if let Some(ref regex) = extracted.name
            && !regex.is_match(&name)
        {
            continue;
        }

        let display = match extracted.mode {
            DisplayMode::Filename => name.into_owned(),
            DisplayMode::Absolute => entry_path.display().to_string(),
            DisplayMode::Relative => match entry_path.strip_prefix(base_path) {
                Ok(rel) => {
                    let rel = rel.display().to_string();
                    if rel.is_empty() {
                        base_path
                            .file_name()
                            .map(|s| s.display().to_string())
                            .expect("file_name should never return None because we canonicalize the path")
                    } else {
                        rel
                    }
                }
                Err(_) => continue,
            },
            DisplayMode::RelativePrefixed => match entry_path.strip_prefix(base_path) {
                Ok(rel) => {
                    let prefix = base_path.file_name().and_then(|s| s.to_str());

                    match prefix {
                        Some(prefix) if rel.as_os_str().is_empty() => prefix.to_string(),
                        Some(prefix) => format!("{}/{}", prefix, rel.display()),
                        None => rel.display().to_string(),
                    }
                }
                Err(_) => continue,
            },
        };

        results.push(InputItem::new(display, entry_path.display().to_string()));

        if let Some(limit) = extracted.limit
            && results.len() >= limit
        {
            break;
        }
    }
}

fn main() {
    let extracted = Cli::parse().extract();

    if extracted.paths.is_empty() {
        eprintln!("{}: please provide one or multiple paths", cli::NAME);
        std::process::exit(1);
    }

    let mut results: Vec<InputItem> = Vec::new();

    for path in &extracted.paths {
        extract_results(&mut results, &extracted, path);

        if let Some(limit) = extracted.limit
            && results.len() >= limit
        {
            break;
        }
    }

    InputItems::new(results).print()
}
