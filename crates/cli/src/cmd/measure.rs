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
    pub async fn execute(&self) -> anyhow::Result<()> {
        // Use an empty slice if "all" is specified, otherwise use the clients vector
        let clients_to_run = if self.clients.len() == 1 && self.clients[0] == "all" {
            &[][..]
        } else {
            &self.clients
        };

        run(clients_to_run).await;

        Ok(())
    }
}
