//! Testing utilities for RUEST.

use crate::core::{bootstrap, Module, RuestApplication};
use crate::di::{Container, DiError};
use crate::http::axum::Router;

/// Factory for integration tests.
pub struct TestFactory;

impl TestFactory {
    pub fn create<M: Module>(root: M) -> Result<RuestApplication, crate::core::CoreError> {
        bootstrap(root)
    }

    /// Monte les routes via la fonction `wire_routes` générée par `#[module]`.
    pub fn create_with_router<M, W>(
        root: M,
        wire: W,
    ) -> Result<(RuestApplication, Router), crate::core::CoreError>
    where
        M: Module,
        W: FnOnce(Router, &Container) -> Result<Router, DiError>,
    {
        let app = bootstrap(root)?;
        let router = wire(Router::new(), &app.container)
            .map_err(|e| crate::core::CoreError::ModuleConfig(e.to_string()))?;
        Ok((app, router))
    }
}
