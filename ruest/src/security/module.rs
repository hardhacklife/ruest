use std::sync::Arc;

use crate::di::Container;

use super::config::SecurityConfig;
use super::jwt::JwtService;
use super::SecurityError;

/// Enregistre `JwtService` dans le conteneur DI (à appeler depuis un `#[module]` ou au bootstrap).
pub fn register_jwt_provider(
    container: &Container,
    config: SecurityConfig,
) -> Result<Arc<JwtService>, SecurityError> {
    JwtService::register_provider(container, config)?;
    container
        .get::<JwtService>()
        .map_err(|e| SecurityError::Config(e.to_string()))
}
