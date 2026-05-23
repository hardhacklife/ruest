use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("{0}")]
    Message(String),

    #[error("not found")]
    NotFound,

    #[error("internal server error")]
    Internal,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FieldError>>,
}

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl HttpError {
    pub fn status(&self) -> StatusCode {
        match self {
            HttpError::NotFound => StatusCode::NOT_FOUND,
            HttpError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::Message(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let status = self.status();
        let body = ErrorBody {
            status: status.as_u16(),
            message: self.to_string(),
            errors: None,
        };
        (status, axum::Json(body)).into_response()
    }
}
