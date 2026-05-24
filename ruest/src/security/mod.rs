//! # RUEST Security
//!
//! Authentification JWT, gardes (`Guard`), extracteur [`AuthUser`], middleware HTTP.
//!
//! ## Démarrage rapide
//!
//! ```ignore
//! use ruest::prelude::*;
//! use ruest::security::{SecurityConfig, JwtService, AuthUser};
//!
//! let config = SecurityConfig::dev();
//!
//! bootstrap_app(AppModule)?
//!     .with_jwt_auth(config)?   // enregistre JwtService + middleware
//!     .port(3000)
//!     .listen()
//!     .await?;
//!
//! // Handler protégé :
//! async fn me(user: AuthUser) -> AppResult<Json<serde_json::Value>> {
//!     Ok(Json(json!({ "sub": user.subject() })))
//! }
//! ```

mod claims;
mod config;
mod context;
mod error;
mod extract;
mod guard;
mod jwt;
mod layer;
mod module;
mod provider;

pub use claims::RuestClaims;
pub use config::{SecurityConfig, SecurityConfigBuilder};
pub use context::AuthContext;
pub use error::SecurityError;
pub use extract::AuthUser;
pub use guard::{run_guards, Guard, JwtGuard, RolesGuard};
pub use jwt::JwtService;
pub use layer::{apply_jwt_layer, jwt_auth_middleware, JwtAuthState};
pub use module::register_jwt_provider;
pub use provider::JwtDevProvider;
