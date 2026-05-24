//! # RUEST
//!
//! Backend framework for Rust — **Rust + NestJS** (NestJS DX, performance Rust, Axum).
//!
//! Un seul package framework : `cargo add ruest` (+ `ruest-macros` en dépendance transitive).
//! Base de données optionnelle : `cargo add ruest-db`.

mod bootstrap;

pub mod config;
pub mod core;
pub mod di;
pub mod http;
pub mod logger;
pub mod router;
pub mod security;
pub mod testing;
pub mod validation;

pub use async_trait;

pub use bootstrap::{bootstrap_app, AppBuilder, ModuleWireRoutes};
pub use ruest_macros::{
    controller, delete, dto, get, guard, middleware, module, patch, post, put, routes, service,
};

pub use core::{bootstrap, CoreError, HttpModule, Module, RuestApplication, RuestFactory};
pub use di::{Container, Inject, Scope};
pub use http::{
    serve, Body, Bytes, ConnectInfo, Form, Json, MatchedPath, Multipart, OriginalUri, Path, Query,
    State, WebSocket, WebSocketUpgrade,
};
pub use http::{ws, Message as WebSocketMessage};
pub use validation::{Validate, ValidatedJson};
pub use http::{AppError, AppResult};
pub use security::{
    apply_jwt_layer, register_jwt_provider, AuthContext, AuthUser, Guard, JwtDevProvider,
    JwtGuard, JwtService, RolesGuard, RuestClaims, SecurityConfig, SecurityConfigBuilder,
    SecurityError,
};

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
        apply_jwt_layer, bootstrap_app, controller, delete, get, guard, module, patch, post, put,
        register_jwt_provider, routes, service, async_trait, ruest_err, AppBuilder, AppError,
        AppResult, AuthContext, AuthUser, Body, Bytes, ConnectInfo, CoreError, Form, HttpModule,
        Inject, Json, JwtDevProvider, JwtGuard, JwtService, MatchedPath, Module, ModuleWireRoutes,
        Multipart, OriginalUri, Path, Query, RolesGuard, RuestApplication, RuestClaims,
        SecurityConfig, SecurityConfigBuilder, SecurityError, State, Validate, ValidatedJson,
        WebSocket, WebSocketMessage, WebSocketUpgrade, logger,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ModuleMetadata;

    struct EmptyModule;

    impl Module for EmptyModule {
        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata::default()
        }
    }

    impl ModuleWireRoutes for EmptyModule {
        fn wire_routes(
            router: http::axum::Router,
            _container: &di::Container,
        ) -> Result<http::axum::Router, di::DiError> {
            Ok(router)
        }
    }

    #[test]
    fn bootstrap_empty_module() {
        let _ = bootstrap_app(EmptyModule).expect("bootstrap");
    }
}
