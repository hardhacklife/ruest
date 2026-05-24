use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config error: {0}")]
    Message(String),

    #[error(transparent)]
    Config(#[from] config::ConfigError),
}
