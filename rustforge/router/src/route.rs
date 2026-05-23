use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use http::Method;

/// HTTP methods supported by RustForge routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head,
}

impl From<HttpMethod> for Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Patch => Method::PATCH,
            HttpMethod::Delete => Method::DELETE,
            HttpMethod::Options => Method::OPTIONS,
            HttpMethod::Head => Method::HEAD,
        }
    }
}

/// A registered route with metadata.
#[derive(Clone)]
pub struct RouteDefinition {
    pub method: HttpMethod,
    pub path: String,
    pub handler_name: &'static str,
    pub handler: Arc<dyn RouteHandler>,
}

/// Type-erased async route handler.
#[async_trait]
pub trait RouteHandler: Send + Sync {
    async fn handle(&self, request: axum::extract::Request) -> axum::response::Response;
}

/// Helper to wrap a function as a route handler.
pub type HandlerFuture =
    Pin<Box<dyn Future<Output = axum::response::Response> + Send>>;

pub fn boxed_handler<F, Fut>(f: F) -> Arc<dyn RouteHandler>
where
    F: Fn(axum::extract::Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = axum::response::Response> + Send + 'static,
{
    struct FnHandler<F>(F);

    #[async_trait]
    impl<F, Fut> RouteHandler for FnHandler<F>
    where
        F: Fn(axum::extract::Request) -> Fut + Send + Sync,
        Fut: Future<Output = axum::response::Response> + Send,
    {
        async fn handle(&self, request: axum::extract::Request) -> axum::response::Response {
            (self.0)(request).await
        }
    }

    Arc::new(FnHandler(f))
}
