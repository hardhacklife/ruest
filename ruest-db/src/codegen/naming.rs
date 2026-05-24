use crate::schema::{Field, FieldKind, Model, ScalarType};

pub fn table_name(model: &str) -> String {
    let snake = to_snake(model);
    pluralize(&snake)
}

pub fn column_name(field: &str) -> String {
    to_snake(field)
}

pub fn rust_module(model: &str) -> String {
    to_snake(model)
}

pub fn rust_struct(model: &str) -> String {
    model.to_string()
}

pub fn create_input_name(model: &str) -> String {
    format!("Create{model}")
}

pub fn update_input_name(model: &str) -> String {
    format!("Update{model}")
}

pub fn delegate_name(model: &str) -> String {
    format!("{model}Delegate")
}

pub fn pg_type(field: &Field) -> String {
    match &field.kind {
        FieldKind::Scalar(t) => match t {
            ScalarType::String => "TEXT".into(),
            ScalarType::Int => "INTEGER".into(),
            ScalarType::Float => "DOUBLE PRECISION".into(),
            ScalarType::Boolean => "BOOLEAN".into(),
            ScalarType::DateTime => "TIMESTAMPTZ".into(),
            ScalarType::Uuid => "UUID".into(),
        },
        FieldKind::Model(_) => "TEXT".into(),
    }
}

pub fn default_sql(field: &Field) -> Option<String> {
    use crate::schema::{Attribute, DefaultValue};

    for attr in &field.attributes {
        if let Attribute::Default(d) = attr {
            return Some(match d {
                DefaultValue::Uuid => "gen_random_uuid()".into(),
                DefaultValue::Now => "NOW()".into(),
                DefaultValue::Literal(v) => format!("'{v}'"),
            });
        }
    }
    None
}

pub fn is_db_column(field: &Field) -> bool {
    matches!(field.kind, FieldKind::Scalar(_))
}

fn to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn pluralize(s: &str) -> String {
    if s.ends_with('y') && s.len() > 1 && !"aeiou".contains(s.as_bytes()[s.len() - 2] as char) {
        format!("{}ies", &s[..s.len() - 1])
    } else if s.ends_with('s') || s.ends_with('x') || s.ends_with("ch") || s.ends_with("sh") {
        format!("{s}es")
    } else {
        format!("{s}s")
    }
}

pub fn table_columns(model: &Model) -> Vec<&Field> {
    model.fields.iter().filter(|f| is_db_column(f)).collect()
}
