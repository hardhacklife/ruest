use super::{ModuleContext, RuestApplication};

/// Plugin extension point (auth, cache, telemetry, etc.).
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;

    fn apply(&self, app: &mut RuestApplication, ctx: &mut ModuleContext)
        -> Result<(), super::CoreError>;
}
