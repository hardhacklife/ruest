use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiError {
    #[error(
        "[Forge DI] Service `{type_name}` is not registered.\n\
         Did you forget to add it to your module?\n\
         \n\
         Try:\n\
         #[module(controllers = [...], providers = [{type_name}])]\n\
         pub struct YourModule;\n\
         \n\
         And ensure `#[service]` is on `{type_name}` with `impl Default` or `register_provider`."
    )]
    NotFound { type_name: &'static str },

    #[error("[Forge DI] Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("[Forge DI] Failed to resolve `{type_name}`: {reason}")]
    ResolutionFailed {
        type_name: &'static str,
        reason: String,
    },
}

impl DiError {
    pub fn not_found<T: 'static>() -> Self {
        Self::NotFound {
            type_name: std::any::type_name::<T>(),
        }
    }
}
