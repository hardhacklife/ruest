use crate::di::Container;

use super::jwt::JwtService;

/// Provider DI pour enregistrer un [`JwtService`] avec [`crate::SecurityConfig::dev`].
pub struct JwtDevProvider;

impl JwtDevProvider {
    pub fn register_provider(container: &Container) {
        JwtService::register_dev_provider(container);
    }
}
