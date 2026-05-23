use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::http::Request;
use tower::{Layer, Service};

pub type RequestBody = Body;

/// Axum request alias used in middleware signatures.
pub type ForgeRequest = Request<RequestBody>;

/// Response type for middleware chain.
pub type ForgeResponse = axum::response::Response;

/// Next handler in the middleware pipeline.
pub struct Next<S> {
    inner: S,
}

impl<S> Next<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}

/// Middleware trait (NestJS / Axum style).
pub trait Middleware: Send + Sync + 'static {
    fn handle(
        &self,
        request: ForgeRequest,
        next: Next<axum::routing::Route>,
    ) -> Pin<Box<dyn Future<Output = ForgeResponse> + Send>>;
}

/// Logging middleware example.
#[derive(Default, Clone)]
pub struct LoggerMiddleware;

impl<S> Layer<S> for LoggerMiddleware {
    type Service = LoggerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggerService { inner }
    }
}

#[derive(Clone)]
pub struct LoggerService<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for LoggerService<S>
where
    S: Service<Request<ReqBody>> + Clone + Send + 'static,
    S::Future: Send,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        tracing::info!(method = %req.method(), uri = %req.uri(), "incoming request");
        self.inner.call(req)
    }
}
