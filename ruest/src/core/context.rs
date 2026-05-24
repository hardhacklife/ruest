use crate::di::Container;

/// Mutable context passed during module configuration (providers + imports).
pub struct ModuleContext {
    pub container: Container,
}

impl ModuleContext {
    pub fn new(container: Container) -> Self {
        Self { container }
    }
}
