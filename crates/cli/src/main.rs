use anyhow::Result;
use clap::{CommandFactory, Parser};
use odometer::cmd::{version::VersionArgs, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        return VersionArgs {}.run();
    }

    if let Some(cmd) = cli.cmd {
        match cmd {}
    } else {
        // Print
        Cli::command().print_help()?;
        println!();
    }

    Ok(())
}
