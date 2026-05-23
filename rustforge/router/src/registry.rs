use crate::{HttpMethod, RouteDefinition};

/// Collects route definitions before they are mounted on the HTTP server.
#[derive(Default, Clone)]
pub struct RouteRegistry {
    routes: Vec<RouteDefinition>,
    global_prefix: Option<String>,
}

impl RouteRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_global_prefix(&mut self, prefix: impl Into<String>) {
        self.global_prefix = Some(prefix.into());
    }

    pub fn register(&mut self, route: RouteDefinition) {
        self.routes.push(route);
    }

    pub fn register_many(&mut self, routes: impl IntoIterator<Item = RouteDefinition>) {
        self.routes.extend(routes);
    }

    pub fn routes(&self) -> &[RouteDefinition] {
        &self.routes
    }

    pub fn resolve_path(&self, path: &str) -> String {
        match &self.global_prefix {
            Some(prefix) => {
                let prefix = prefix.trim_end_matches('/');
                let path = if path.starts_with('/') {
                    path.to_string()
                } else {
                    format!("/{path}")
                };
                format!("{prefix}{path}")
            }
            None => {
                if path.starts_with('/') {
                    path.to_string()
                } else {
                    format!("/{path}")
                }
            }
        }
    }

    pub fn find(&self, method: HttpMethod, path: &str) -> Option<&RouteDefinition> {
        self.routes
            .iter()
            .find(|r| r.method == method && r.path == path)
    }
}
