//! Testing utilities for RUEST.

use ruest_core::{bootstrap, Module, RuestApplication};
use ruest_di::{Container, DiError};
use ruest_http::axum::Router;

/// Factory for integration tests.
pub struct TestFactory;

impl TestFactory {
    pub fn create<M: Module>(root: M) -> Result<RuestApplication, ruest_core::CoreError> {
        bootstrap(root)
    }

    /// Monte les routes via la fonction `wire_routes` générée par `#[module]`.
    pub fn create_with_router<M, W>(
        root: M,
        wire: W,
    ) -> Result<(RuestApplication, Router), ruest_core::CoreError>
    where
        M: Module,
        W: FnOnce(Router, &Container) -> Result<Router, DiError>,
    {
        let app = bootstrap(root)?;
        let router = wire(Router::new(), &app.container)
            .map_err(|e| ruest_core::CoreError::ModuleConfig(e.to_string()))?;
        Ok((app, router))
    }
}
