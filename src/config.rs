use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub app: AppSettings,
    pub http: HttpSettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppSettings {
    pub name: String,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HttpSettings {
    pub timeout_seconds: u64,
    pub user_agent: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: AppSettings {
                name: "mycli".to_string(),
                log_level: "info".to_string(),
            },
            http: HttpSettings {
                timeout_seconds: 30,
                user_agent: "mycli/0.1.0".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let mut builder = config::Config::builder();

        // 1. Default values
        builder = builder.add_source(config::Config::try_from(&AppConfig::default())?);

        // 2. Config file
        let path = config_path
            .map(|p| p.to_path_buf())
            .unwrap_or_else(default_config_path);

        if path.exists() {
            builder = builder.add_source(config::File::from(path.as_path()));
        }

        // 3. Environment variables (MYCLI_APP__NAME, MYCLI_HTTP__TIMEOUT_SECONDS, etc.)
        builder = builder.add_source(
            config::Environment::with_prefix("MYCLI")
                .separator("__")
                .try_parsing(true),
        );

        builder
            .build()
            .context("Failed to build config")?
            .try_deserialize()
            .context("Failed to deserialize config")
    }

    pub fn config_path() -> PathBuf {
        default_config_path()
    }
}

fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("mycli")
        .join("config.toml")
}
