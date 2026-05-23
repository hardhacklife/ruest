//! Core runtime for RustForge: application lifecycle, modules, and bootstrap.

mod app;
mod context;
mod error;
mod factory;
mod module;
mod plugin;

pub use app::{bootstrap, RustForgeApplication};
pub use context::ModuleContext;
pub use error::CoreError;
pub use factory::{ApplicationBuilder, RustForgeFactory};
pub use module::{HttpModule, Module, ModuleMetadata, ProviderRegisterFn};
pub use plugin::Plugin;
