//! Génération SQL et code Rust à partir de [`ruest_db_schema::Schema`].

mod naming;
mod rust_client;
mod sql;

pub use naming::table_name;
pub use rust_client::generate_client;
pub use sql::{generate_create_all, generate_migration_sql};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("{0}")]
    Message(String),
}
