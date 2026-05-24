//! Baseline Axum — même réponse JSON que `basic-api` GET /users, sans RUEST.
//!
//! Port par défaut : 3001 (basic-api utilise 3000).

use axum::{routing::get, Json, Router};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct User {
    id: Uuid,
    email: String,
    name: String,
}

async fn get_users() -> Json<Vec<User>> {
    Json(vec![User {
        id: Uuid::new_v4(),
        email: "demo@ruest.dev".into(),
        name: "Demo User".into(),
    }])
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".into());
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/users/", get(get_users));

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    eprintln!("axum-baseline listening on http://127.0.0.1:{port}");
    axum::serve(listener, app).await.expect("serve");
}
