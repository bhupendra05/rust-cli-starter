use anyhow::Result;
use colored::Colorize;

use crate::config::AppConfig;

pub fn run(config: &AppConfig, show_path: bool, json_output: bool) -> Result<()> {
    if show_path {
        println!("{}", AppConfig::config_path().display());
        return Ok(());
    }

    if json_output {
        println!("{}", serde_json::to_string_pretty(config)?);
        return Ok(());
    }

    println!("{}", "Current Configuration".bold().underline());
    println!();
    println!("  {} {}", "Config file:".dim(), AppConfig::config_path().display());
    println!();
    println!("  {} {}", "[app]".cyan().bold(), "");
    println!("    name       = {}", config.app.name);
    println!("    log_level  = {}", config.app.log_level);
    println!();
    println!("  {} {}", "[http]".cyan().bold(), "");
    println!("    timeout    = {}s", config.http.timeout_seconds);
    println!("    user_agent = {}", config.http.user_agent);

    Ok(())
}
