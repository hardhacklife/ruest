//! Extracteurs HTTP réexportés depuis Axum (toutes les features activées au niveau workspace).

// --- Corps / JSON / formulaires ---
pub use axum::extract::{DefaultBodyLimit, Form, Json};
pub use axum::body::{Body, Bytes};
pub use axum::RequestExt;

// --- Paramètres de route / query ---
pub use axum::extract::{Path, Query, State};

// --- Multipart ---
pub use axum::extract::Multipart;

// --- Métadonnées requête (features matched-path, original-uri) ---
pub use axum::extract::{MatchedPath, OriginalUri};

// --- WebSocket (feature ws) ---
pub use axum::extract::ws::{self, Message, WebSocket, WebSocketUpgrade};

// --- Connexion client (feature tokio) ---
pub use axum::extract::connect_info::{self, ConnectInfo};

// --- Extension / headers ---
pub use axum::extract::Extension;
pub use axum::http::header;
