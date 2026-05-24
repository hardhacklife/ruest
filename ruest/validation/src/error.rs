use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("validation failed")]
    Failed(ValidationErrors),
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorBody {
    pub status: u16,
    pub message: String,
    pub errors: Vec<ValidationFieldError>,
}

#[derive(Debug, Serialize)]
pub struct ValidationFieldError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn from_errors(errors: ValidationErrors) -> Self {
        Self::Failed(errors)
    }
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        let ValidationError::Failed(errors) = self;
        let field_errors: Vec<ValidationFieldError> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errs)| {
                errs.iter().map(move |e| ValidationFieldError {
                    field: field.to_string(),
                    message: e
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "invalid".into()),
                })
            })
            .collect();

        let body = ValidationErrorBody {
            status: StatusCode::BAD_REQUEST.as_u16(),
            message: "Validation failed".into(),
            errors: field_errors,
        };

        (StatusCode::BAD_REQUEST, axum::Json(body)).into_response()
    }
}
