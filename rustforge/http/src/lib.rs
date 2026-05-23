//! HTTP server layer for RustForge (Axum + Tower, features complètes).
//!
//! Axum est compilé avec HTTP/1, HTTP/2, WebSocket, multipart, form, query, etc.
//! Voir [`axum_features`] pour le détail.

pub use axum;

pub mod axum_features;

mod error;
mod extract;
mod middleware;
mod response;
mod result;
mod server;

pub use error::HttpError;
pub use extract::{
    connect_info, header, ws, Body, Bytes, ConnectInfo, DefaultBodyLimit, Extension, Form, Json,
    MatchedPath, Message, Multipart, OriginalUri, Path, Query, RequestExt, State, WebSocket,
    WebSocketUpgrade,
};
/// Requête HTTP (`http::Request<Body>`).
pub use axum::http::Request;
pub use middleware::{Middleware, Next};
pub use response::ApiResponse;
pub use result::{AppError, AppResult};
pub use server::serve;
