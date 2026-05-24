//! Bootstrap HTTP : assemble le routeur Axum au compile-time via `Module::wire_routes`.

use crate::core::{bootstrap, CoreError, Module, RuestApplication};
use crate::http::axum::Router;
use crate::security::{apply_jwt_layer, JwtService, SecurityConfig};

/// Application prête à écouter (DI + routeur Axum monomorphisé).
pub struct AppBuilder {
    pub app: RuestApplication,
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
        container: &crate::di::Container,
    ) -> Result<Router, crate::di::DiError>;
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
        crate::http::serve(self.app, self.router)
            .await
            .map_err(|e| CoreError::Bootstrap(e.to_string()))
    }

    pub fn build(self) -> (RuestApplication, Router) {
        (self.app, self.router)
    }

    /// Active l'authentification JWT (enregistre [`JwtService`] si besoin + middleware).
    ///
    /// Enregistrez `JwtService` plus tôt via `JwtService::register_dev_provider` ou
    /// `register_jwt_provider` dans un `#[module]` si des contrôleurs l'injectent au câblage des routes.
    pub fn with_jwt_auth(mut self, config: SecurityConfig) -> Result<Self, CoreError> {
        if self.app.container.get::<JwtService>().is_err() {
            crate::security::register_jwt_provider(&self.app.container, config.clone())
                .map_err(|e| CoreError::ModuleConfig(e.to_string()))?;
        }

        let jwt = self
            .app
            .container
            .get::<JwtService>()
            .map_err(|e| CoreError::ModuleConfig(e.to_string()))?;

        self.router = apply_jwt_layer(self.router, jwt, &config);
        Ok(self)
    }
}
