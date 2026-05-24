use ruest_db_schema::{Attribute, FieldKind, Schema};

use crate::naming::{column_name, default_sql, pg_type, table_columns, table_name};

/// SQL `CREATE TABLE` pour tous les modèles (ordre du schema).
pub fn generate_create_all(schema: &Schema) -> String {
    let mut parts = Vec::new();
    for model in &schema.models {
        parts.push(generate_create_table(model));
    }
    parts.join("\n\n")
}

pub fn generate_migration_sql(schema: &Schema) -> String {
    format!(
        "-- RuestDB migration\n-- Generated from schema.ruest\n\n{}\n",
        generate_create_all(schema)
    )
}

fn generate_create_table(model: &ruest_db_schema::Model) -> String {
    let table = table_name(&model.name);
    let mut lines = Vec::new();

    for field in table_columns(model) {
        let col = column_name(&field.name);
        let mut line = format!("  \"{col}\" {}", pg_type(field));
        if field
            .attributes
            .iter()
            .any(|a| matches!(a, Attribute::Id))
        {
            line.push_str(" PRIMARY KEY");
        }
        if field
            .attributes
            .iter()
            .any(|a| matches!(a, Attribute::Unique))
        {
            line.push_str(" UNIQUE");
        }
        if !field.optional {
            line.push_str(" NOT NULL");
        }
        if let Some(def) = default_sql(field) {
            line.push_str(&format!(" DEFAULT {def}"));
        }
        lines.push(line);
    }

    // FK depuis @relation (champ relationnel → colonne FK déjà déclarée)
    for field in &model.fields {
        let FieldKind::Model(target) = &field.kind else {
            continue;
        };
        let Some(rel) = field.attributes.iter().find_map(|a| match a {
            Attribute::Relation(r) => Some(r),
            _ => None,
        }) else {
            continue;
        };
        let fk_col = column_name(&rel.fields[0]);
        let ref_table = table_name(target);
        let ref_col = column_name(&rel.references[0]);
        lines.push(format!(
            "  CONSTRAINT \"fk_{table}_{fk_col}\" FOREIGN KEY (\"{fk_col}\") REFERENCES \"{ref_table}\" (\"{ref_col}\") ON DELETE CASCADE"
        ));
    }

    format!(
        "CREATE TABLE IF NOT EXISTS \"{table}\" (\n{}\n);",
        lines.join(",\n")
    )
}

#[cfg(test)]
mod tests {
    use ruest_db_parser::parse_schema;
    use super::*;

    const SAMPLE: &str = r#"
model User {
  id    String @id @default(uuid())
  email String @unique
  name  String
  posts Post[]
}

model Post {
  id     String @id @default(uuid())
  title  String
  userId String
  user   User @relation(fields: [userId], references: [id])
}
"#;

    #[test]
    fn generates_postgres_ddl() {
        let schema = parse_schema(SAMPLE).unwrap();
        let sql = generate_create_all(&schema);
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS \"users\""));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS \"posts\""));
        assert!(sql.contains("FOREIGN KEY"));
    }
}
