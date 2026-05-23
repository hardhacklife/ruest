//! Bootstrap HTTP : assemble le routeur Axum au compile-time via `Module::wire_routes`.

use rustforge_core::{bootstrap, CoreError, Module, RustForgeApplication};
use rustforge_http::axum::Router;

/// Application prête à écouter (DI + routeur Axum monomorphisé).
pub struct AppBuilder {
    pub app: RustForgeApplication,
    pub router: Router,
}

/// Bootstrap DI + montage des routes compile-time.
///
/// Nécessite que `#[module]` ait généré `wire_routes` sur le type module.
pub fn bootstrap_app<M>(root: M) -> Result<AppBuilder, CoreError>
where
    M: Module + ModuleWireRoutes,
{
    let app = bootstrap(root)?;
    let router = M::wire_routes(Router::new(), &app.container)
        .map_err(|e| CoreError::ModuleConfig(e.to_string()))?;
    Ok(AppBuilder { app, router })
}

/// Trait généré implicitement par `#[module]` (via `wire_routes` inherent).
pub trait ModuleWireRoutes {
    fn wire_routes(
        router: Router,
        container: &rustforge_di::Container,
    ) -> Result<Router, rustforge_di::DiError>;
}

impl AppBuilder {
    pub fn port(mut self, port: u16) -> Self {
        self.app.set_port(port);
        self
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.app.set_host(host);
        self
    }

    pub async fn listen(self) -> Result<(), CoreError> {
        rustforge_http::serve(self.app, self.router)
            .await
            .map_err(|e| CoreError::Bootstrap(e.to_string()))
    }

    pub fn build(self) -> (RustForgeApplication, Router) {
        (self.app, self.router)
    }
}
