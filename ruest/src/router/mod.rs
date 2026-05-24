//! HTTP routing primitives for RUEST.

mod path;
mod route;
mod registry;

pub use path::join_paths;
pub use route::{boxed_handler, HttpMethod, RouteDefinition, RouteHandler};
pub use registry::RouteRegistry;
