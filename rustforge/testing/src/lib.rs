//! Testing utilities for RustForge.

use rustforge_core::{bootstrap, Module, RustForgeApplication};
use rustforge_di::{Container, DiError};
use rustforge_http::axum::Router;

/// Factory for integration tests.
pub struct TestFactory;

impl TestFactory {
    pub fn create<M: Module>(root: M) -> Result<RustForgeApplication, rustforge_core::CoreError> {
        bootstrap(root)
    }

    /// Monte les routes via la fonction `wire_routes` générée par `#[module]`.
    pub fn create_with_router<M, W>(
        root: M,
        wire: W,
    ) -> Result<(RustForgeApplication, Router), rustforge_core::CoreError>
    where
        M: Module,
        W: FnOnce(Router, &Container) -> Result<Router, DiError>,
    {
        let app = bootstrap(root)?;
        let router = wire(Router::new(), &app.container)
            .map_err(|e| rustforge_core::CoreError::ModuleConfig(e.to_string()))?;
        Ok((app, router))
    }
}
