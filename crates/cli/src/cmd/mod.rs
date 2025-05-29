//! Command implementations for Odometer CLI

pub mod version;

pub use version::*;

use clap::Parser;

/// Odometer: Ethereum client benchmarking tool
#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    disable_version_flag= true,
)]
pub struct Cli {
    #[clap(short = 'v', long = "version", global = true)]
    pub version: bool,

    #[clap(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Debug, Parser)]
pub enum Commands {}
