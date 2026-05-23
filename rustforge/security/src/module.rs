use std::sync::Arc;

use rustforge_di::Container;

use crate::config::SecurityConfig;
use crate::jwt::JwtService;
use crate::SecurityError;

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
