mod cli;

use clap::Parser;
use common::{InputItem, InputItems};
use kymenu_ssh::SSHHosts;
use regex::Regex;

use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let compile_rgx = |name: &str, rgx: Option<String>| {
        rgx.map(|rgx| {
            Regex::new(&rgx).unwrap_or_else(|e| {
                eprintln!("{}: failed to compile {name} '{rgx}': '{e}'", cli::NAME);
                std::process::exit(1)
            })
        })
    };

    let name_rgx = compile_rgx("name_rgx", cli.name_rgx);
    let user_rgx = compile_rgx("user_rgx", cli.user_rgx);
    let host_rgx = compile_rgx("host_rgx", cli.host_rgx);

    let hosts = SSHHosts::new(cli.user_config, cli.system_config)?;

    let mut result = Vec::new();

    for (name, config) in hosts.0 {
        if let Some(rgx) = &name_rgx
            && !rgx.is_match(&name)
        {
            continue;
        }

        if let Some(username) = &config.user
            && let Some(user_rgx) = &user_rgx
            && !user_rgx.is_match(username)
        {
            continue;
        }

        if let Some(hostname) = &config.hostname
            && let Some(host_rgx) = &host_rgx
            && !host_rgx.is_match(hostname)
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
