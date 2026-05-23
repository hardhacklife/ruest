/// Lifetime scope for a registered provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Scope {
    /// One instance for the entire application.
    #[default]
    Singleton,
    /// A new instance on every resolution.
    Transient,
    /// One instance per HTTP request (stored in request extensions).
    Request,
}
