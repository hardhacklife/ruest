use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiError {
    #[error("provider not registered for type: {0}")]
    NotFound(&'static str),

    #[error("circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("failed to resolve provider: {0}")]
    ResolutionFailed(String),
}
