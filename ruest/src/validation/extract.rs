use axum::async_trait;
use axum::extract::{FromRequest, Request};
use axum::Json;
use serde::de::DeserializeOwned;
use validator::Validate;

use super::ValidationError;

/// JSON extractor that runs `validator` automatically.
pub struct ValidatedJson<T>(pub T);

impl<T> std::ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| ValidationError::from_errors(validator::ValidationErrors::new()))?;

        value
            .validate()
            .map_err(ValidationError::from_errors)?;

        Ok(ValidatedJson(value))
    }
}
