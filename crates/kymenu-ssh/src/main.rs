mod cli;

use clap::Parser;
use common::{InputItem, InputItems};
use kymenu_ssh::SSHHosts;
use regex::Regex;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let regex = cli.regex.map(|regex| {
        Regex::new(&regex).unwrap_or_else(|e| {
            eprintln!("{}: failed to compile regex '{regex}': '{e}'", cli::NAME);
            std::process::exit(1)
        })
    });

    let hosts = SSHHosts::new(cli.user_config, cli.system_config)?;

    let mut result = Vec::new();

    for (name, config) in hosts.0 {
        if let Some(regex) = &regex
            && !regex.is_match(&name)
        {
            continue;
        }

        let username = config.user.unwrap_or("?".to_string());
        let hostname = config.hostname.unwrap_or("?".to_string());

        let display = match (cli.username, cli.hostname) {
            (true, true) => format!("{name} ({username}@{hostname})"),
            (true, false) => format!("{name} ({username})"),
            (false, true) => format!("{name} ({hostname})"),
            (false, false) => name.clone(),
        };

        result.push(InputItem::new(display, name));
    }

    InputItems::new(result).print();

    Ok(())
}
