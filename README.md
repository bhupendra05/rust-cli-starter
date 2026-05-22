# rust-cli-starter

> Production-ready Rust CLI template. Clone → rename → build. Ships with clap v4, tokio, tracing, config, colored output, Docker, and CI.

![Rust](https://img.shields.io/badge/rust-1.78+-orange) ![License](https://img.shields.io/badge/license-MIT-green) ![CI](https://github.com/bhupendra05/rust-cli-starter/actions/workflows/ci.yml/badge.svg)

```
$ mycli hello Bhupendra --count 3
[1/3] Hello, Bhupendra!
[2/3] Hello, Bhupendra!
[3/3] Hello, Bhupendra!

$ mycli fetch https://httpbin.org/get --headers
Status: 200 OK

Headers:
  content-type: application/json
  ...

$ mycli config
Current Configuration
  Config file: ~/.config/mycli/config.toml
  [app]
    name      = mycli
    log_level = info
  [http]
    timeout   = 30s
```

## What's included

| Feature | Crate | Notes |
|---------|-------|-------|
| CLI parsing | `clap` v4 | derive macros, subcommands, env vars |
| Async runtime | `tokio` | full feature set |
| Structured logging | `tracing` + `tracing-subscriber` | env-filter, JSON mode |
| Error handling | `anyhow` + `thiserror` | library vs. binary errors |
| Configuration | `config` | TOML file + env vars with `MYCLI__` prefix |
| Colored output | `colored` | cross-platform, respects `NO_COLOR` |
| Progress spinners | `indicatif` | spinner for long operations |
| HTTP client | `reqwest` | async, rustls, JSON |
| Docker | multi-stage | builder + slim runtime, non-root user |
| CI | GitHub Actions | test + clippy + fmt, macOS/Linux/Windows matrix |

## Quick Start

### Use as template

Click **"Use this template"** on GitHub, or:

```bash
git clone https://github.com/bhupendra05/rust-cli-starter.git myapp
cd myapp
# Rename the binary
sed -i 's/mycli/myapp/g' Cargo.toml src/cli.rs
cargo build
```

### Build and run

```bash
cargo build           # debug build
cargo build --release # optimized binary → target/release/mycli
cargo run -- hello    # run directly
```

### Install globally

```bash
cargo install --path .
mycli --help
```

### Docker

```bash
docker build -t mycli .
docker run --rm mycli hello --count 3
```

---

## Project Structure

```
rust-cli-starter/
├── src/
│   ├── main.rs            # Entry point — arg parse, init, dispatch
│   ├── cli.rs             # Clap Cli struct + Commands enum + dispatch fn
│   ├── config.rs          # AppConfig — TOML + env var loading
│   ├── error.rs           # thiserror CliError enum with exit codes
│   ├── output.rs          # tracing init, success/error/info helpers
│   └── commands/
│       ├── mod.rs
│       ├── hello.rs       # Example sync command
│       ├── fetch.rs       # Example async command (HTTP)
│       └── config.rs      # Show current config
├── Dockerfile             # Multi-stage: builder + slim runtime
├── config.example.toml    # Config file template
├── Cargo.toml
└── .github/workflows/
    └── ci.yml             # Test + clippy + fmt + security audit
```

## Adding a New Command

1. Create `src/commands/mycommand.rs`:

```rust
use anyhow::Result;

pub async fn run(arg: &str) -> Result<()> {
    println!("Running with: {}", arg);
    Ok(())
}
```

2. Add to `src/commands/mod.rs`:

```rust
pub mod mycommand;
```

3. Add to the `Commands` enum in `src/cli.rs`:

```rust
/// My new command
MyCommand {
    arg: String,
},
```

4. Add dispatch arm:

```rust
Commands::MyCommand { arg } => {
    commands::mycommand::run(&arg).await
}
```

## Configuration

Config is loaded from (in priority order):

1. Defaults (hardcoded in `AppConfig::default()`)
2. Config file: `~/.config/mycli/config.toml` (or `--config path/to/file.toml`)
3. Environment variables: `MYCLI__APP__LOG_LEVEL=debug` (double underscore separator)

```bash
# Copy example config
mkdir -p ~/.config/mycli
cp config.example.toml ~/.config/mycli/config.toml

# Or use env vars
MYCLI__HTTP__TIMEOUT_SECONDS=60 mycli fetch https://slow-api.example.com
```

## Logging

```bash
mycli -v hello        # INFO level
mycli -vv hello       # DEBUG level
mycli -vvv hello      # TRACE level

# JSON logs for production
mycli --json-output hello

# Override with RUST_LOG
RUST_LOG=debug mycli hello
```

## License

MIT © bhupendra05
