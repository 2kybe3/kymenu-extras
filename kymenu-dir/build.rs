use clap::CommandFactory;
use clap_complete::Shell;
use clap_complete::generate_to;
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Cli::command();
    let name = cmd.get_name().to_owned();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, &name, &outdir)?;
    }

    Ok(())
}
