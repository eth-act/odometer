//! The version command

use clap::Args;

#[derive(Debug, Args)]
pub struct VersionArgs {}

impl VersionArgs {
    pub fn run(self) -> Result<(), anyhow::Error> {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        let description = env!("CARGO_PKG_DESCRIPTION");

        println!("{} v{}", name, version);
        println!("{}", description);

        println!("Platform: {}", std::env::consts::OS);

        Ok(())
    }
}
