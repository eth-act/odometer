use anyhow::Result;
use clap::{CommandFactory, Parser};
use odometer::cmd::{version::VersionArgs, Cli, Commands, MeasureCommands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        return VersionArgs {}.run();
    }

    if let Some(cmd) = cli.cmd {
        match cmd {
            Commands::Measure(measure_cmd) => match measure_cmd {
                MeasureCommands::GasLimit(gas_limit_cmd) => gas_limit_cmd.execute()?,
            },
        }
    } else {
        // If no command is provided, print help
        Cli::command().print_help()?;
        println!();
    }

    Ok(())
}
