use std::{collections::HashSet, path::PathBuf};

use regex::Regex;

use crate::cli::{self, Cli, DisplayMode};

pub(crate) struct Extracted {
    pub(crate) paths: HashSet<PathBuf>,
    pub(crate) mode: DisplayMode,
    pub(crate) max_depth: Option<usize>,
    pub(crate) min_depth: usize,
    pub(crate) file: bool,
    pub(crate) folder: bool,
    pub(crate) exclude: Vec<String>,
    pub(crate) hidden: bool,
    pub(crate) limit: Option<usize>,
    pub(crate) ext: HashSet<String>,
    pub(crate) file_rgx: Option<Regex>,
    pub(crate) path_rgx: Option<Regex>,
}

impl Cli {
    pub(crate) fn extract(self) -> Extracted {
        let paths: HashSet<PathBuf> = self
            .paths
            .iter()
            .map(|path| path.canonicalize().unwrap_or(path.to_owned()))
            .collect();

        let mode = self.mode.unwrap_or(if paths.len() == 1 {
            DisplayMode::Relative
        } else {
            DisplayMode::RelativePrefixed
        });

        let compile_regex = |regex_opt: Option<String>| {
            regex_opt.and_then(|regex| {
                Regex::new(&regex)
                    .inspect_err(|e| {
                        eprintln!("{}: regex '{regex}' failed to compile: '{e}'", cli::NAME)
                    })
                    .ok()
            })
        };

        Extracted {
            paths,
            mode,
            max_depth: self.max_depth,
            min_depth: self.min_depth.unwrap_or(
                if let Some(max_depth) = self.max_depth
                    && max_depth == 0
                {
                    0
                } else {
                    1
                },
            ),
            file: self.file,
            folder: self.folder,
            exclude: self.exclude,
            hidden: self.hidden,
            limit: self.limit,
            ext: self
                .ext
                .iter()
                .map(|e| e.trim_start_matches(".").to_owned())
                .collect(),
            file_rgx: compile_regex(self.file_rgx),
            path_rgx: compile_regex(self.path_rgx),
        }
    }
}
