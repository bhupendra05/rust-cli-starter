use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::Result;

use crate::commands;
use crate::config::AppConfig;

/// mycli — A production-ready Rust CLI (replace this with your description)
#[derive(Parser, Debug)]
#[command(
    name = "mycli",
    version,
    author,
    about,
    long_about = None,
    propagate_version = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to config file (default: ~/.config/mycli/config.toml)
    #[arg(long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Enable verbose logging (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Output as JSON (machine-readable)
    #[arg(long, global = true)]
    pub json_output: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Say hello (example command)
    Hello {
        /// Name to greet
        #[arg(value_name = "NAME", default_value = "World")]
        name: String,

        /// Number of times to greet
        #[arg(short, long, default_value = "1")]
        count: u32,
    },

    /// Fetch data from a URL (example async command)
    Fetch {
        /// URL to fetch
        url: String,

        /// Show response headers
        #[arg(short = 'H', long)]
        headers: bool,
    },

    /// Show current configuration
    Config {
        /// Show config file path
        #[arg(long)]
        path: bool,
    },
}

pub async fn dispatch(cli: Cli, config: AppConfig) -> Result<()> {
    match cli.command {
        Commands::Hello { name, count } => {
            commands::hello::run(&name, count, cli.json_output)
        }
        Commands::Fetch { url, headers } => {
            commands::fetch::run(&url, headers, cli.json_output).await
        }
        Commands::Config { path } => {
            commands::config::run(&config, path, cli.json_output)
        }
    }
}
