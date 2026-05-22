use anyhow::Result;
use colored::Colorize;
use serde_json::json;

pub fn run(name: &str, count: u32, json_output: bool) -> Result<()> {
    if json_output {
        let output = json!({
            "greeting": format!("Hello, {}!", name),
            "count": count,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    for i in 1..=count {
        if count > 1 {
            println!(
                "[{}/{}] {} {}{}",
                i,
                count,
                "Hello,".cyan(),
                name.bold(),
                "!".cyan()
            );
        } else {
            println!("{} {}!", "Hello,".cyan(), name.bold());
        }
    }

    Ok(())
}
