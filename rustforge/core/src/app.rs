use rustforge_di::Container;

use crate::{CoreError, Module, ModuleContext, Plugin};

/// Built application (container + config). Le routeur Axum est assemblé au bootstrap HTTP.
pub struct RustForgeApplication {
    pub container: Container,
    pub port: u16,
    pub host: String,
    plugins: Vec<Box<dyn Plugin>>,
}

impl RustForgeApplication {
    pub fn new(container: Container) -> Self {
        Self {
            container,
            port: 3000,
            host: "0.0.0.0".into(),
            plugins: Vec::new(),
        }
    }

    pub fn use_plugin(&mut self, plugin: impl Plugin + 'static) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    pub fn set_host(&mut self, host: impl Into<String>) -> &mut Self {
        self.host = host.into();
        self
    }

    pub(crate) fn apply_plugins(&mut self, _ctx: &mut ModuleContext) -> Result<(), CoreError> {
        Ok(())
    }
}

/// Configure le conteneur DI et les providers (sans montage HTTP).
pub fn bootstrap<M: Module>(root: M) -> Result<RustForgeApplication, CoreError> {
    let container = Container::new();
    let mut ctx = ModuleContext::new(container.clone());
    root.configure(&mut ctx)?;
    Ok(RustForgeApplication::new(ctx.container))
}
