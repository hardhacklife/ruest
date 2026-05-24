use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use crate::core::RuestApplication;
use crate::di::Container;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use super::error::HttpError;
use super::middleware::LoggerMiddleware;

/// Build and serve a pre-assembled Axum router (routes monomorphisées au compile-time).
pub async fn serve(app: RuestApplication, router: Router) -> Result<(), HttpError> {
    let built = finalize_router(app, router)?;
    let addr: SocketAddr = format!("{}:{}", built.host, built.port)
        .parse()
        .map_err(|e: std::net::AddrParseError| HttpError::Message(e.to_string()))?;

    tracing::info!("RUEST listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e: std::io::Error| HttpError::Message(e.to_string()))?;

    axum::serve(listener, built.router)
        .await
        .map_err(|e: std::io::Error| HttpError::Message(e.to_string()))?;

    Ok(())
}

struct BuiltRouter {
    router: Router,
    host: String,
    port: u16,
}

fn finalize_router(app: RuestApplication, router: Router) -> Result<BuiltRouter, HttpError> {
    let container = Arc::new(app.container);

    let router = router
        .route("/health", get(|| async { "ok" }))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .layer(LoggerMiddleware)
        .layer(axum::Extension(ContainerExtension(Arc::clone(&container))));

    Ok(BuiltRouter {
        router,
        host: app.host,
        port: app.port,
    })
}

/// DI container dans les extensions de requête (zero-copy : `Arc` partagé).
#[derive(Clone)]
pub struct ContainerExtension(pub Arc<Container>);
