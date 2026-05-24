//! Dependency injection container for RUEST.

mod container;
mod error;
mod inject;
mod provider;
mod scope;

pub use container::Container;
pub use error::DiError;
pub use inject::Inject;
pub use provider::{default_provider, FactoryProvider, Provider, ProviderDescriptor};
pub use scope::Scope;
