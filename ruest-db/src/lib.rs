//! RuestDB — schéma `schema.ruest`, migrations et client Rust type-safe (SQLx).
//!
//! Un seul package : `cargo add ruest-db`
//!
//! ```ignore
//! use ruest_db::{RuestDb, db_init, generate_client, parse_schema};
//! ```

pub mod codegen;
pub mod migrate;
pub mod parser;
pub mod runtime;
pub mod schema;

pub use codegen::{generate_migration_sql, table_name};
pub use migrate::{
    create_migration, db_init, generate_client, migrate_apply, migrate_reset, MigrateError,
    MIGRATIONS_DIR, SCHEMA_FILE,
};
pub use parser::{parse_schema, ParseError};
pub use runtime::{RuestDb, RuestDbError};
pub use schema::{
    Attribute, DefaultValue, Field, FieldKind, Model, RelationAttr, ScalarType, Schema,
};

/// Réexportés pour le code généré (`generated/ruestdb/`).
pub use runtime::{chrono, serde, sqlx, uuid, Row};
