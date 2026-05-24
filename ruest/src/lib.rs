//! # RUEST
//!
//! Backend framework for Rust — **NestJS DX + performance Rust + simplicité Axum**.
//!
//! Voir [ARCHITECTURE.md](../ARCHITECTURE.md) (performance) et [docs/PRINCIPES.md](../docs/PRINCIPES.md)
//! (modulaire, typé, sécurisé, fonctionnel).

mod bootstrap;

pub mod core {
    pub use ruest_core::*;
}
pub mod di {
    pub use ruest_di::*;
}
pub mod http {
    pub use ruest_http::*;
}
pub mod router {
    pub use ruest_router::*;
}
pub mod config {
    pub use ruest_config::*;
}
pub mod validation {
    pub use ruest_validation::*;
}
pub mod logger {
    pub use ruest_logger::*;
}
pub mod testing {
    pub use ruest_testing::*;
}
pub mod security {
    pub use ruest_security::*;
}

pub use async_trait;

pub use bootstrap::{bootstrap_app, AppBuilder, ModuleWireRoutes};
pub use ruest_macros::{
    controller, delete, dto, get, guard, middleware, module, patch, post, put, routes, service,
};

pub use ruest_core::{bootstrap, CoreError, HttpModule, Module, RuestApplication, RuestFactory};
pub use ruest_di::{Container, Inject, Scope};
pub use ruest_http::{
    serve, Body, Bytes, ConnectInfo, Form, Json, MatchedPath, Multipart, OriginalUri, Path, Query,
    State, WebSocket, WebSocketUpgrade,
};
pub use ruest_http::{ws, Message as WebSocketMessage};
pub use ruest_validation::{Validate, ValidatedJson};
pub use ruest_http::{AppError, AppResult};

/// Erreur HTTP lisible : `return Err(ruest_err!(BadRequest, "message"));`
#[macro_export]
macro_rules! ruest_err {
    (BadRequest, $msg:expr) => {
        $crate::AppError::bad_request($msg)
    };
    (NotFound, $msg:expr) => {
        $crate::AppError::not_found($msg)
    };
    (Conflict, $msg:expr) => {
        $crate::AppError::conflict($msg)
    };
    (Internal, $msg:expr) => {
        $crate::AppError::internal($msg)
    };
    (Unauthorized, $msg:expr) => {
        $crate::AppError::unauthorized($msg)
    };
    (Forbidden, $msg:expr) => {
        $crate::AppError::forbidden($msg)
    };
}

/// Prelude for application code.
pub mod prelude {
    pub use crate::{
        bootstrap_app, controller, delete, get, guard, module, patch, post, put, routes, service,
        async_trait, ruest_err, AppBuilder, AppError, AppResult, Body, Bytes, ConnectInfo,
        CoreError, Form, HttpModule, Inject, Json, MatchedPath, Module, ModuleWireRoutes,
        Multipart, OriginalUri, Path, Query, RuestApplication, State, Validate, ValidatedJson,
        WebSocket, WebSocketMessage, WebSocketUpgrade, logger,
    };
    pub use crate::security::{
        apply_jwt_layer, register_jwt_provider, AuthContext, AuthUser, RuestClaims, Guard,
        JwtDevProvider, JwtGuard, JwtService, RolesGuard, SecurityConfig, SecurityConfigBuilder,
        SecurityError,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruest_core::ModuleMetadata;

    struct EmptyModule;

    impl Module for EmptyModule {
        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata::default()
        }
    }

    impl ModuleWireRoutes for EmptyModule {
        fn wire_routes(
            router: ruest_http::axum::Router,
            _container: &ruest_di::Container,
        ) -> Result<ruest_http::axum::Router, ruest_di::DiError> {
            Ok(router)
        }
    }

    #[test]
    fn bootstrap_empty_module() {
        let builder = bootstrap_app(EmptyModule).expect("bootstrap");
        assert_eq!(builder.app.port, 3000);
    }
}
