use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::Http(_) => 2,
            CliError::Config(_) => 3,
            CliError::Io(_) => 4,
            CliError::Other(_) => 1,
        }
    }
}
