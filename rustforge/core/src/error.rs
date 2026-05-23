use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("module configuration failed: {0}")]
    ModuleConfig(String),

    #[error("bootstrap failed: {0}")]
    Bootstrap(String),

    #[error(transparent)]
    Di(#[from] rustforge_di::DiError),
}
