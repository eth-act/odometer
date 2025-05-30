//! Measure command implementation for benchmarking different metrics

use clap::Parser;
use profiler::core::run;

#[derive(Debug, Parser)]
#[clap(about = "Measure performance metrics", disable_help_subcommand = true)]
pub enum MeasureCommands {
    #[clap(name = "gas-limit")]
    GasLimit(GasLimitCmd),
}

#[derive(Debug, Parser)]
pub struct GasLimitCmd {
    #[clap(
        long = "for",
        value_delimiter = ',',
        default_value = "all",
        help = "Specify comma-separated client names to measure gas limit for. Use 'all' for all clients."
    )]
    pub clients: Vec<String>,
}

impl GasLimitCmd {
    /// Execute the gas limit measurement command
    pub async fn execute(&self) -> anyhow::Result<()> {
        run().await;

        Ok(())
    }
}
