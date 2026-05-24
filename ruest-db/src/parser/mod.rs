//! Parse un fichier `schema.ruest` vers [`crate::schema::Schema`].

mod parse;

pub use parse::{parse_schema, ParseError};
