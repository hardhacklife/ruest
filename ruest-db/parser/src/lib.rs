//! Parse un fichier `schema.ruest` vers [`ruest_db_schema::Schema`].

mod parse;

pub use parse::{parse_schema, ParseError};
