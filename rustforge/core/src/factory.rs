use crate::{bootstrap, CoreError, Module, RustForgeApplication};

/// Factory for creating RustForge applications (NestJS-style).
pub struct RustForgeFactory;

impl RustForgeFactory {
    /// Create and bootstrap an application from the root module.
    pub fn create<M: Module>(root: M) -> Result<ApplicationBuilder, CoreError> {
        let app = bootstrap(root)?;
        Ok(ApplicationBuilder { app })
    }
}

/// Fluent builder after module bootstrap.
pub struct ApplicationBuilder {
    app: RustForgeApplication,
}

impl ApplicationBuilder {
    pub fn port(mut self, port: u16) -> Self {
        self.app.set_port(port);
        self
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.app.set_host(host);
        self
    }

    pub fn build(self) -> RustForgeApplication {
        self.app
    }
}
