use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseSSHHostsError {
    #[error("Error opening config file {0}: {1}")]
    OpenConfigFileError(PathBuf, std::io::Error),
    #[error("Error reading config file {0}: {1}")]
    ReadConfigFileError(PathBuf, std::io::Error),
    #[error("{0}")]
    EnvError(std::env::VarError),
}

#[derive(Debug)]
pub struct SSHHostSetting {
    pub user: Option<String>,
    pub hostname: Option<String>,
}

impl SSHHostSetting {
    pub fn new(user: Option<String>, hostname: Option<String>) -> Self {
        Self { user, hostname }
    }
}

pub struct SSHHosts(pub HashMap<String, SSHHostSetting>);

#[derive(Debug, Clone)]
pub struct SSHConfigItem {
    setting: String,
    value: String,
}

impl SSHConfigItem {
    pub fn new(setting: &str, value: &str) -> Self {
        Self {
            setting: setting.trim().to_lowercase().to_owned(),
            value: value.trim().to_owned(),
        }
    }
}

struct PendingHost {
    names: Vec<String>,
    user: Option<String>,
    hostname: Option<String>,
}

impl PendingHost {
    fn new() -> Self {
        Self {
            names: Vec::new(),
            user: None,
            hostname: None,
        }
    }

    fn set_host(&mut self, value: &str) {
        self.names = value.split_whitespace().map(|s| s.to_string()).collect();
    }

    fn clear(&mut self) {
        self.names.clear();
        self.user = None;
        self.hostname = None;
    }

    fn flush_pending(&mut self, results: &mut HashMap<String, SSHHostSetting>) {
        for name in &self.names {
            if name.starts_with('!')
                || name.contains('*')
                || name.contains('?')
                || name.contains('[')
                || name.contains(']')
            {
                continue;
            }

            results.insert(
                name.to_owned(),
                SSHHostSetting::new(self.user.clone(), self.hostname.clone()),
            );
        }

        self.clear();
    }
}

impl SSHHosts {
    pub fn new(user_config: bool, system_config: bool) -> Result<Self, ParseSSHHostsError> {
        let mut visited = HashSet::new();

        let mut res = HashMap::new();

        if user_config {
            let user_config = PathBuf::from(format!(
                "{}/.ssh/config",
                std::env::var("HOME").map_err(ParseSSHHostsError::EnvError)?
            ));
            let user_config = Self::read_config_file(&user_config, &mut visited)?;
            res.extend(Self::process_items(&user_config));
        }

        if system_config {
            let system_config =
                Self::read_config_file(Path::new("/etc/ssh/ssh_config"), &mut visited)?;
            res.extend(Self::process_items(&system_config));
        }

        Ok(SSHHosts(res))
    }

    fn process_items(items: &[SSHConfigItem]) -> HashMap<String, SSHHostSetting> {
        let mut results = HashMap::new();

        let mut pending = PendingHost::new();
        for item in items {
            match item.setting.as_str() {
                "host" => {
                    pending.flush_pending(&mut results);

                    pending.set_host(&item.value);
                }
                "user" => pending.user = Some(item.value.clone()),
                "hostname" => pending.hostname = Some(item.value.clone()),
                _ => {}
            }
        }
        pending.flush_pending(&mut results);

        results
    }

    fn read_config_file(
        path: &Path,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<Vec<SSHConfigItem>, ParseSSHHostsError> {
        let canonical = fs::canonicalize(path)
            .map_err(|e| ParseSSHHostsError::OpenConfigFileError(path.to_owned(), e))?;

        if !visited.insert(canonical.clone()) {
            return Ok(vec![]);
        }

        let content = fs::read_to_string(&canonical)
            .map_err(|e| ParseSSHHostsError::ReadConfigFileError(path.to_owned(), e))?;

        let mut items = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            let item = match line.split_once(" ") {
                Some((setting, value)) => SSHConfigItem::new(setting, value),
                None => continue,
            };

            if item.setting == "include" {
                let include_path = canonical.parent().unwrap().join(&item.value);
                items.append(&mut Self::read_config_file(&include_path, visited)?);
            }

            items.push(item);
        }

        Ok(items)
    }
}
