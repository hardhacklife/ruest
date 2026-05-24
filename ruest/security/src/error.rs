use ruest_http::AppError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("JWT configuration error: {0}")]
    Config(String),

    #[error("invalid or expired token")]
    InvalidToken,

    #[error("missing Authorization header")]
    MissingAuthorization,

    #[error("invalid Authorization scheme (expected Bearer)")]
    InvalidScheme,

    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

impl SecurityError {
    pub fn into_app_error(self) -> AppError {
        match self {
            Self::InvalidToken | Self::MissingAuthorization | Self::InvalidScheme => {
                AppError::unauthorized(self.to_string())
            }
            Self::Config(msg) => AppError::internal(msg),
            Self::Jwt(e) => AppError::unauthorized(format!("token rejected: {e}")),
        }
    }
}
