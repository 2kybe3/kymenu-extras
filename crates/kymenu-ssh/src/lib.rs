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

    fn flush_pending(&mut self, results: &mut Vec<Rule>) {
        for name in &self.names {
            results.push(Rule {
                pattern: name.to_owned(),
                user: self.user.clone(),
                hostname: self.hostname.clone(),
            });
        }

        self.clear();
    }
}

struct Rule {
    pattern: String,
    user: Option<String>,
    hostname: Option<String>,
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

    fn resolve(name: &str, rules: &[Rule]) -> SSHHostSetting {
        let mut user = None;
        let mut hostname = None;

        for rule in rules {
            if ssh_matcher::Matcher::new(&rule.pattern).matches(name) {
                if rule.user.is_some() {
                    user = rule.user.clone();
                }

                if rule.hostname.is_some() {
                    hostname = rule.hostname.clone();
                }
            }
        }

        SSHHostSetting::new(user, hostname)
    }

    fn generate_rules(items: &[SSHConfigItem]) -> Vec<Rule> {
        let mut results = Vec::new();

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

    fn process_items(items: &[SSHConfigItem]) -> HashMap<String, SSHHostSetting> {
        let mut results = HashMap::new();

        let rules = Self::generate_rules(items);

        for rule in &rules {
            let name = &rule.pattern;

            if name.contains("?") || name.contains("*") || name.contains("[") || name.contains("]")
            {
                continue;
            }

            results.insert(name.clone(), Self::resolve(name, &rules));
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matchers() {
        let mut visited = HashSet::new();

        let res = SSHHosts::process_items(
            &SSHHosts::read_config_file(&PathBuf::from("test_data/matchers/config"), &mut visited)
                .unwrap(),
        );

        assert_eq!(res.len(), 5);
        assert!(res.contains_key("test"));
        assert!(res.contains_key("test1"));
        assert!(res.contains_key("test2"));
        assert!(res.contains_key("abc1"));
        assert!(res.contains_key("abc2"));

        let test = res.get("test").unwrap();
        assert_eq!(test.user, Some("abc".to_string()));
        assert_eq!(test.hostname, None);

        let test1 = res.get("test1").unwrap();
        assert_eq!(test1.user, Some("test".to_string()));
        assert_eq!(test1.hostname, Some("test.example.com".to_string()));

        let test2 = res.get("test2").unwrap();
        assert_eq!(test2.user, Some("test".to_string()));
        assert_eq!(test2.hostname, None);

        let abc1 = res.get("abc1").unwrap();
        assert_eq!(abc1.user, Some("abc".to_string()));
        assert_eq!(abc1.hostname, None);

        let abc2 = res.get("abc2").unwrap();
        assert_eq!(abc2.user, Some("abc".to_string()));
        assert_eq!(abc2.hostname, None);
    }

    #[test]
    fn basic() {
        let mut visited = HashSet::new();

        let res = SSHHosts::process_items(
            &SSHHosts::read_config_file(&PathBuf::from("test_data/basic/config"), &mut visited)
                .unwrap(),
        );

        assert_eq!(res.len(), 6);
        assert!(res.contains_key("test"));
        assert!(res.contains_key("test1"));
        assert!(res.contains_key("test2"));
        assert!(res.contains_key("test3"));
        assert!(res.contains_key("no_host"));
        assert!(res.contains_key("no_user"));

        let test = res.get("test").unwrap();
        assert_eq!(test.user, Some("testing2".to_string()));
        assert_eq!(test.hostname, Some("host_name.example.com".to_string()));

        let test1 = res.get("test1").unwrap();
        assert_eq!(test1.user, Some("testing2".to_string()));
        assert_eq!(test1.hostname, None);

        let test2 = res.get("test2").unwrap();
        assert_eq!(test2.user, Some("testing2".to_string()));
        assert_eq!(test2.hostname, None);

        let test3 = res.get("test3").unwrap();
        assert_eq!(test3.user, Some("testing".to_string()));
        assert_eq!(test3.hostname, Some("host_name.example.com".to_string()));

        let no_user = res.get("no_user").unwrap();
        assert_eq!(no_user.user, None);
        assert_eq!(no_user.hostname, Some("no_user.example.com".to_string()));

        let no_host = res.get("no_host").unwrap();
        assert_eq!(no_host.user, Some("no_host".to_string()));
        assert_eq!(no_host.hostname, None);
    }
}
