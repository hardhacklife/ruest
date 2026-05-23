use crate::{ModuleContext, RustForgeApplication};

/// Plugin extension point (auth, cache, telemetry, etc.).
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;

    fn apply(&self, app: &mut RustForgeApplication, ctx: &mut ModuleContext)
        -> Result<(), crate::CoreError>;
}
