use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init_tracing(verbose: u8, json: bool) {
    let level = match verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    if json {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_target(false))
            .init();
    }
}

/// Print a success message in green
pub fn success(msg: &str) {
    use colored::Colorize;
    println!("{} {}", "✓".green().bold(), msg);
}

/// Print an error message in red
pub fn error(msg: &str) {
    use colored::Colorize;
    eprintln!("{} {}", "✗".red().bold(), msg);
}

/// Print an info message in cyan
pub fn info(msg: &str) {
    use colored::Colorize;
    println!("{} {}", "→".cyan(), msg);
}
