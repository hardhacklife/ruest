//! Documentation des capacités Axum activées par RustForge.
//!
//! Le workspace active **toutes** les features optionnelles d'Axum 0.7 listées ci-dessous.
//! Les applications n'ont pas à réactiver ces flags sur leur propre dépendance `axum` si elles
//! passent par `rustforge::http`.

/// Features Axum activées dans `Cargo.toml` workspace :
///
/// | Feature | Description |
/// |---------|-------------|
/// | `http1` | HTTP/1.1 via Hyper |
/// | `http2` | HTTP/2 via Hyper |
/// | `json` | [`Json`](crate::Json) et helpers JSON |
/// | `macros` | Macros utilitaires Axum (`#[debug_handler]`, etc.) |
/// | `matched-path` | [`MatchedPath`](crate::MatchedPath) — chemin routeur capturé |
/// | `multipart` | [`Multipart`](crate::Multipart) — upload fichiers |
/// | `original-uri` | [`OriginalUri`](crate::OriginalUri) — URI d'origine |
/// | `tokio` | `axum::serve`, SSE, [`ConnectInfo`](crate::ConnectInfo) |
/// | `tower-log` | Logs Tower |
/// | `tracing` | Logs des rejets des extracteurs intégrés |
/// | `ws` | WebSockets [`WebSocketUpgrade`](crate::WebSocketUpgrade) |
/// | `form` | [`Form`](crate::Form) — `application/x-www-form-urlencoded` |
/// | `query` | [`Query`](crate::Query) — query string |
pub mod docs {}
