use forgedb_schema::{
    Attribute, DefaultValue, Field, FieldKind, Model, RelationAttr, ScalarType, Schema,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("parse error at line {line}: {message}")]
    Syntax { line: usize, message: String },
}

pub fn parse_schema(source: &str) -> Result<Schema, ParseError> {
    let mut models = Vec::new();
    let mut line_no = 0usize;

    let lines: Vec<&str> = source.lines().map(str::trim).collect();
    let mut i = 0;

    while i < lines.len() {
        line_no = i + 1;
        let line = lines[i];
        i += 1;

        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if let Some(name) = line.strip_prefix("model ").and_then(|s| s.strip_suffix('{')) {
            let name = name.trim().to_string();
            let (body, consumed) = read_block(&lines[i..], line_no)?;
            i += consumed;
            models.push(parse_model(&name, &body)?);
            continue;
        }

        return Err(ParseError::Syntax {
            line: line_no,
            message: format!("expected `model Name {{`, got `{line}`"),
        });
    }

    validate_relations(&models)?;
    Ok(Schema { models })
}

fn read_block(lines: &[&str], start_line: usize) -> Result<(Vec<String>, usize), ParseError> {
    let mut body = Vec::new();
    let mut depth = 1usize;
    let mut i = 0usize;

    while i < lines.len() {
        let line = lines[i];
        i += 1;
        for ch in line.chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
            }
        }
        if depth == 0 {
            let trimmed = line.trim_end_matches('}').trim();
            if !trimmed.is_empty() {
                body.push(trimmed.to_string());
            }
            return Ok((body, i));
        }
        body.push(line.to_string());
    }

    Err(ParseError::Syntax {
        line: start_line,
        message: "unclosed model block".into(),
    })
}

fn parse_model(name: &str, body: &[String]) -> Result<Model, ParseError> {
    let mut fields = Vec::new();

    for (idx, line) in body.iter().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        fields.push(parse_field(line).map_err(|message| ParseError::Syntax {
            line: idx + 1,
            message,
        })?);
    }

    if fields.is_empty() {
        return Err(ParseError::Syntax {
            line: 0,
            message: format!("model `{name}` has no fields"),
        });
    }

    Ok(Model {
        name: name.to_string(),
        fields,
    })
}

fn parse_field(line: &str) -> Result<Field, String> {
    let (head, attrs) = split_attributes(line);
    let parts: Vec<&str> = head.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(format!("invalid field line: `{line}`"));
    }

    let name = parts[0].to_string();
    let mut type_part = parts[1];
    let optional = type_part.ends_with('?');
    if optional {
        type_part = &type_part[..type_part.len() - 1];
    }
    let list = type_part.ends_with("[]");
    let type_name = if list {
        &type_part[..type_part.len() - 2]
    } else {
        type_part
    };

    let kind = parse_type_name(type_name)?;
    let attributes = parse_attributes(&attrs)?;

    Ok(Field {
        name,
        kind,
        optional,
        list,
        attributes,
    })
}

fn split_attributes(line: &str) -> (&str, &str) {
    if let Some(pos) = line.find('@') {
        (&line[..pos], &line[pos..])
    } else {
        (line, "")
    }
}

fn parse_type_name(name: &str) -> Result<FieldKind, String> {
    match name {
        "String" => Ok(FieldKind::Scalar(ScalarType::String)),
        "Int" => Ok(FieldKind::Scalar(ScalarType::Int)),
        "Float" => Ok(FieldKind::Scalar(ScalarType::Float)),
        "Boolean" => Ok(FieldKind::Scalar(ScalarType::Boolean)),
        "DateTime" => Ok(FieldKind::Scalar(ScalarType::DateTime)),
        "UUID" | "Uuid" => Ok(FieldKind::Scalar(ScalarType::Uuid)),
        other if other.chars().next().is_some_and(|c| c.is_ascii_uppercase()) => {
            Ok(FieldKind::Model(other.to_string()))
        }
        other => Err(format!("unknown type `{other}`")),
    }
}

fn parse_attributes(src: &str) -> Result<Vec<Attribute>, String> {
    let mut attrs = Vec::new();
    let mut rest = src.trim();

    while let Some(stripped) = rest.strip_prefix('@') {
        rest = stripped;
        let (name, rem) = take_ident(rest);
        rest = rem.trim();

        match name {
            "id" => attrs.push(Attribute::Id),
            "unique" => attrs.push(Attribute::Unique),
            "default" => {
                let (value, rem) = parse_paren_value(rest)?;
                rest = rem.trim();
                attrs.push(Attribute::Default(parse_default(&value)?));
            }
            "relation" => {
                let (inner, rem) = parse_paren_value(rest)?;
                rest = rem.trim();
                attrs.push(Attribute::Relation(parse_relation(&inner)?));
            }
            other => return Err(format!("unknown attribute `@{other}`")),
        }
    }

    Ok(attrs)
}

fn take_ident(s: &str) -> (&str, &str) {
    let end = s
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .unwrap_or(s.len());
    (&s[..end], &s[end..])
}

fn parse_paren_value(s: &str) -> Result<(String, &str), String> {
    let s = s.trim();
    if !s.starts_with('(') {
        return Err("expected `(` after attribute".into());
    }
    let mut depth = 0i32;
    let mut end = 0usize;
    for (i, ch) in s.char_indices() {
        if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
            if depth == 0 {
                end = i;
                break;
            }
        }
    }
    if depth != 0 {
        return Err("unclosed parentheses".into());
    }
    Ok((s[1..end].to_string(), &s[end + 1..]))
}

fn parse_default(value: &str) -> Result<DefaultValue, String> {
    let value = value.trim();
    if value == "uuid()" {
        return Ok(DefaultValue::Uuid);
    }
    if value == "now()" {
        return Ok(DefaultValue::Now);
    }
    if (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        return Ok(DefaultValue::Literal(
            value[1..value.len() - 1].to_string(),
        ));
    }
    Ok(DefaultValue::Literal(value.to_string()))
}

fn parse_relation(inner: &str) -> Result<RelationAttr, String> {
    let mut fields = Vec::new();
    let mut references = Vec::new();
    let mut current: Option<&str> = None;

    for part in inner.split(',') {
        let part = part.trim();
        if let Some(key) = part.strip_prefix("fields:") {
            current = Some("fields");
            let key = key.trim();
            if !key.is_empty() {
                fields.extend(parse_bracket_list(key)?);
            }
            continue;
        }
        if let Some(key) = part.strip_prefix("references:") {
            current = Some("references");
            let key = key.trim();
            if !key.is_empty() {
                references.extend(parse_bracket_list(key)?);
            }
            continue;
        }
        if part.starts_with('[') {
            let list = parse_bracket_list(part)?;
            match current {
                Some("fields") => fields.extend(list),
                Some("references") => references.extend(list),
                _ => return Err(format!("unexpected list in relation: `{part}`")),
            }
        }
    }

    if fields.is_empty() || references.is_empty() {
        return Err("relation requires fields and references".into());
    }

    Ok(RelationAttr { fields, references })
}

fn parse_bracket_list(s: &str) -> Result<Vec<String>, String> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return Err(format!("expected bracket list, got `{s}`"));
    }
    let inner = &s[1..s.len() - 1];
    Ok(inner
        .split(',')
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect())
}

fn validate_relations(models: &[Model]) -> Result<(), ParseError> {
    let names: std::collections::HashSet<_> = models.iter().map(|m| m.name.as_str()).collect();

    for model in models {
        for field in &model.fields {
            if let FieldKind::Model(ref target) = field.kind {
                if !names.contains(target.as_str()) {
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!(
                            "model `{}` references unknown model `{target}`",
                            model.name
                        ),
                    });
                }
            }
            if field.is_relation_scalar() {
                if let Some(Attribute::Relation(rel)) = field
                    .attributes
                    .iter()
                    .find(|a| matches!(a, Attribute::Relation(_)))
                {
                    for fk in &rel.fields {
                        if !model.fields.iter().any(|f| f.name == *fk) {
                            return Err(ParseError::Syntax {
                                line: 0,
                                message: format!(
                                    "relation on `{}` references missing field `{fk}`",
                                    model.name
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
model User {
  id        String   @id @default(uuid())
  email     String   @unique
  name      String
  posts     Post[]
}

model Post {
  id        String @id @default(uuid())
  title     String
  userId    String
  user      User @relation(fields: [userId], references: [id])
}
"#;

    #[test]
    fn parses_prisma_like_schema() {
        let schema = parse_schema(SAMPLE).expect("parse");
        assert_eq!(schema.models.len(), 2);
        assert!(schema.model("User").unwrap().id_field().is_some());
    }
}
