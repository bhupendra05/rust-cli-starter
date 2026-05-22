mod cli;
mod commands;
mod config;
mod error;
mod output;

use anyhow::Result;
use tracing::debug;

use cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    output::init_tracing(cli.verbose, cli.json_output);

    debug!("Starting CLI with args: {:?}", std::env::args().collect::<Vec<_>>());

    // Load config
    let cfg = config::AppConfig::load(cli.config.as_deref())?;

    // Dispatch command
    cli::dispatch(cli, cfg).await?;

    Ok(())
}
