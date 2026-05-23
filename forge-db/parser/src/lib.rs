//! Parse un fichier `schema.forge` vers [`forgedb_schema::Schema`].

mod parse;

pub use parse::{parse_schema, ParseError};
