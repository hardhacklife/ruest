//! # RustForge
//!
//! Backend framework for Rust — **NestJS DX + performance Rust + simplicité Axum**.
//!
//! Voir [ARCHITECTURE.md](../ARCHITECTURE.md) pour les choix performance (routing/DI compile-time).

mod bootstrap;

pub mod core {
    pub use rustforge_core::*;
}
pub mod di {
    pub use rustforge_di::*;
}
pub mod http {
    pub use rustforge_http::*;
}
pub mod router {
    pub use rustforge_router::*;
}
pub mod config {
    pub use rustforge_config::*;
}
pub mod validation {
    pub use rustforge_validation::*;
}
pub mod logger {
    pub use rustforge_logger::*;
}
pub mod testing {
    pub use rustforge_testing::*;
}

pub use bootstrap::{bootstrap_app, AppBuilder, ModuleWireRoutes};
pub use rustforge_macros::{
    controller, delete, dto, get, guard, middleware, module, patch, post, put, routes, service,
};

pub use rustforge_core::{bootstrap, CoreError, HttpModule, Module, RustForgeApplication, RustForgeFactory};
pub use rustforge_di::{Container, Inject, Scope};
pub use rustforge_http::{
    serve, Body, Bytes, ConnectInfo, Form, Json, MatchedPath, Multipart, OriginalUri, Path, Query,
    State, WebSocket, WebSocketUpgrade,
};
pub use rustforge_http::{ws, Message as WebSocketMessage};
pub use rustforge_validation::{Validate, ValidatedJson};
pub use rustforge_http::{AppError, AppResult};

/// Erreur HTTP lisible : `return Err(forge_err!(BadRequest, "message"));`
#[macro_export]
macro_rules! forge_err {
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
}

/// Prelude for application code.
pub mod prelude {
    pub use crate::{
        bootstrap_app, controller, delete, get, module, patch, post, put, routes, service,
        forge_err, AppBuilder, AppError, AppResult, Body, Bytes, ConnectInfo, CoreError, Form,
        HttpModule, Inject, Json, MatchedPath, Module, ModuleWireRoutes, Multipart,
        OriginalUri, Path, Query, RustForgeApplication, State, Validate, ValidatedJson,
        WebSocket, WebSocketMessage, WebSocketUpgrade, logger,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustforge_core::ModuleMetadata;

    struct EmptyModule;

    impl Module for EmptyModule {
        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata::default()
        }
    }

    impl ModuleWireRoutes for EmptyModule {
        fn wire_routes(
            router: rustforge_http::axum::Router,
            _container: &rustforge_di::Container,
        ) -> Result<rustforge_http::axum::Router, rustforge_di::DiError> {
            Ok(router)
        }
    }

    #[test]
    fn bootstrap_empty_module() {
        let builder = bootstrap_app(EmptyModule).expect("bootstrap");
        assert_eq!(builder.app.port, 3000);
    }
}
