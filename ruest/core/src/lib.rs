//! Core runtime for RUEST: application lifecycle, modules, and bootstrap.

mod app;
mod context;
mod error;
mod factory;
mod module;
mod plugin;

pub use app::{bootstrap, RuestApplication};
pub use context::ModuleContext;
pub use error::CoreError;
pub use factory::{ApplicationBuilder, RuestFactory};
pub use module::{HttpModule, Module, ModuleMetadata, ProviderRegisterFn};
pub use plugin::Plugin;
