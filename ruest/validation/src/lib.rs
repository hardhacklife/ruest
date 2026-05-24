//! Request validation for RUEST.

mod error;
mod extract;

pub use error::ValidationError;
pub use extract::ValidatedJson;

pub use validator::Validate;
