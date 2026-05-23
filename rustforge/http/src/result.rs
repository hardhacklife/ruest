//! Résultats et erreurs HTTP lisibles (`AppResult`, pas de `Result<Result<...>>`).

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// Résultat standard d’un handler ou service HTTP.
pub type AppResult<T> = Result<T, AppError>;

/// Erreurs métier avec messages humains (évite les messages Rust cryptiques côté API).
#[derive(Debug, Clone)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

impl AppError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        Self::Conflict(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        Self::Forbidden(msg.into())
    }

    pub fn status(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize)]
struct ErrorBody {
    status: u16,
    message: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::BadRequest(m) => write!(f, "[Forge] Bad Request: {m}"),
            AppError::Unauthorized(m) => write!(f, "[Forge] Unauthorized: {m}"),
            AppError::Forbidden(m) => write!(f, "[Forge] Forbidden: {m}"),
            AppError::NotFound(m) => write!(f, "[Forge] Not Found: {m}"),
            AppError::Conflict(m) => write!(f, "[Forge] Conflict: {m}"),
            AppError::Internal(m) => write!(f, "[Forge] Internal Error: {m}"),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let message = match &self {
            AppError::BadRequest(m)
            | AppError::Unauthorized(m)
            | AppError::Forbidden(m)
            | AppError::NotFound(m)
            | AppError::Conflict(m)
            | AppError::Internal(m) => m.clone(),
        };
        let body = ErrorBody {
            status: status.as_u16(),
            message,
        };
        (status, axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn status_codes_match_http_semantics() {
        assert_eq!(
            AppError::bad_request("x").status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(AppError::not_found("x").status(), StatusCode::NOT_FOUND);
        assert_eq!(AppError::conflict("x").status(), StatusCode::CONFLICT);
        assert_eq!(
            AppError::internal("x").status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn display_is_human_readable() {
        let msg = AppError::not_found("missing").to_string();
        assert!(msg.contains("Not Found"));
        assert!(msg.contains("missing"));
    }
}
