use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use std::time::Duration;
use tracing::debug;

pub async fn run(url: &str, show_headers: bool, json_output: bool) -> Result<()> {
    debug!("Fetching URL: {}", url);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.cyan} {msg}")?,
    );
    pb.set_message(format!("Fetching {}...", url));
    pb.enable_steady_tick(Duration::from_millis(80));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("rust-cli-starter/0.1.0")
        .build()?;

    let response = client.get(url).send().await?;
    pb.finish_and_clear();

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await?;

    if json_output {
        let headers_map: serde_json::Map<String, serde_json::Value> = headers
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    serde_json::Value::String(v.to_str().unwrap_or("").to_string()),
                )
            })
            .collect();

        let output = json!({
            "url": url,
            "status": status.as_u16(),
            "headers": headers_map,
            "body": body,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    // Pretty output
    let status_str = format!("{}", status.as_u16());
    let colored_status = if status.is_success() {
        status_str.green().bold()
    } else if status.is_client_error() {
        status_str.yellow().bold()
    } else {
        status_str.red().bold()
    };

    println!("{} {} {}", "Status:".dim(), colored_status, status.canonical_reason().unwrap_or(""));

    if show_headers {
        println!("\n{}", "Headers:".dim());
        for (key, value) in &headers {
            println!("  {}: {}", key.to_string().cyan(), value.to_str().unwrap_or(""));
        }
    }

    println!("\n{}", "Body:".dim());
    println!("{}", body);

    Ok(())
}
