use forgedb_schema::{Attribute, FieldKind, ScalarType, Schema};

use crate::naming::{
    column_name, create_input_name, delegate_name, rust_module, rust_struct, table_name,
    table_columns, update_input_name,
};

pub struct GeneratedClient {
    pub root: String,
    pub modules: Vec<(String, String)>,
}

/// Génère le client Rust (`generated/forgedb/`).
pub fn generate_client(schema: &Schema) -> GeneratedClient {
    let mut modules = Vec::new();
    let mut delegate_fields = String::new();
    let mut mod_decls = String::new();

    for model in &schema.models {
        let mod_name = rust_module(&model.name);
        let struct_name = rust_struct(&model.name);
        let table = table_name(&model.name);
        let delegate = delegate_name(&model.name);
        let create_name = create_input_name(&model.name);
        let update_name = update_input_name(&model.name);

        mod_decls.push_str(&format!("pub mod {mod_name};\n"));
        delegate_fields.push_str(&format!(
            "    pub {mod_name}: {mod_name}::{delegate},\n"
        ));

        let id = model.id_field().expect("@id required");
        let id_name = &id.name;
        let id_ty = scalar_rust_type(id);
        let id_col = column_name(id_name);

        let cols: Vec<_> = table_columns(model)
            .iter()
            .map(|f| column_name(&f.name))
            .collect();
        let select = cols
            .iter()
            .map(|c| format!("\"{c}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let mut entity_fields = String::new();
        let mut create_fields = String::new();
        let mut update_fields = String::new();
        let mut map_row = String::new();
        let mut insert_cols = Vec::new();
        let mut insert_ph = Vec::new();
        let mut insert_binds = String::new();
        let mut insert_idx = 1i32;

        for field in table_columns(model) {
            let fname = &field.name;
            let ty = scalar_rust_type(field);
            entity_fields.push_str(&format!("    pub {fname}: {ty},\n"));
            map_row.push_str(&format!(
                "            {fname}: row.try_get::<{ty}, _>(\"{fname}\")?,\n"
            ));

            if field.attributes.iter().any(|a| matches!(a, Attribute::Id)) {
                continue;
            }

            let (create_ty, update_ty) = if field.optional {
                (format!("Option<{ty}>"), format!("Option<{ty}>"))
            } else {
                (ty.clone(), format!("Option<{ty}>"))
            };
            create_fields.push_str(&format!("    pub {fname}: {create_ty},\n"));
            update_fields.push_str(&format!("    pub {fname}: {update_ty},\n"));

            insert_cols.push(format!("\"{}\"", column_name(fname)));
            insert_ph.push(format!("${insert_idx}"));
            insert_idx += 1;
            if field.optional {
                insert_binds.push_str(&format!("            .bind(&input.{fname})\n"));
            } else {
                insert_binds.push_str(&format!("            .bind(input.{fname})\n"));
            }
        }

        let insert_cols_s = insert_cols.join(", ");
        let insert_ph_s = insert_ph.join(", ");
        let update_set = generate_update_set_sql(model);

        let find_many_sql = rust_string_literal(&format!(
            "SELECT {select} FROM \"{table}\" ORDER BY \"{id_col}\""
        ));
        let find_unique_sql = rust_string_literal(&format!(
            "SELECT {select} FROM \"{table}\" WHERE \"{id_col}\" = $1"
        ));
        let insert_sql = rust_string_literal(&format!(
            "INSERT INTO \"{table}\" ({insert_cols_s}) VALUES ({insert_ph_s}) RETURNING {select}"
        ));
        let update_sql = rust_string_literal(&format!(
            "UPDATE \"{table}\" SET {update_set} WHERE \"{id_col}\" = $1 RETURNING {select}"
        ));
        let delete_sql = rust_string_literal(&format!(
            "DELETE FROM \"{table}\" WHERE \"{id_col}\" = $1"
        ));

        let module_src = format!(
            r##"//! Généré par ForgeDB — ne pas modifier.

use forgedb_runtime::{{ForgeDb, ForgeDbError}};
use forgedb_runtime::serde::{{Deserialize, Serialize}};
use forgedb_runtime::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {struct_name} {{
{entity_fields}}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {create_name} {{
{create_fields}}}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct {update_name} {{
{update_fields}}}

pub struct {delegate} {{
    db: ForgeDb,
}}

impl {delegate} {{
    pub(crate) fn new(db: ForgeDb) -> Self {{
        Self {{ db }}
    }}

    fn map_row(row: &forgedb_runtime::sqlx::postgres::PgRow) -> Result<{struct_name}, ForgeDbError> {{
        Ok({struct_name} {{
{map_row}        }})
    }}

    pub async fn find_many(&self) -> Result<Vec<{struct_name}>, ForgeDbError> {{
        let sql = {find_many_sql};
        let rows = forgedb_runtime::sqlx::query(sql).fetch_all(self.db.pool()).await?;
        rows.iter().map(Self::map_row).collect()
    }}

    pub async fn find_unique(&self, id: {id_ty}) -> Result<Option<{struct_name}>, ForgeDbError> {{
        let sql = {find_unique_sql};
        let row = forgedb_runtime::sqlx::query(&sql)
            .bind(id)
            .fetch_optional(self.db.pool())
            .await?;
        row.as_ref().map(Self::map_row).transpose()
    }}

    pub async fn create(&self, input: {create_name}) -> Result<{struct_name}, ForgeDbError> {{
        let sql = {insert_sql};
        let row = forgedb_runtime::sqlx::query(sql)
{insert_binds}            .fetch_one(self.db.pool())
            .await?;
        Self::map_row(&row)
    }}

    pub async fn update(
        &self,
        id: {id_ty},
        input: {update_name},
    ) -> Result<Option<{struct_name}>, ForgeDbError> {{
        let existing = self.find_unique(id.clone()).await?;
        let Some(mut current) = existing else {{
            return Ok(None);
        }};
{update_apply}
        let sql = {update_sql};
        let row = forgedb_runtime::sqlx::query(sql)
            .bind(id)
{update_binds}
            .fetch_optional(self.db.pool())
            .await?;
        row.as_ref().map(Self::map_row).transpose()
    }}

    pub async fn delete(&self, id: {id_ty}) -> Result<bool, ForgeDbError> {{
        let sql = {delete_sql};
        let r = forgedb_runtime::sqlx::query(sql).bind(id).execute(self.db.pool()).await?;
        Ok(r.rows_affected() > 0)
    }}
}}
"##,
            update_apply = generate_update_apply(model),
            update_binds = generate_update_binds(model),
            find_many_sql = find_many_sql,
            find_unique_sql = find_unique_sql,
            insert_sql = insert_sql,
            update_sql = update_sql,
            delete_sql = delete_sql,
        );

        modules.push((mod_name, module_src));
    }

    let delegate_inits = schema
        .models
        .iter()
        .map(|m| {
            let mod_name = rust_module(&m.name);
            let delegate = delegate_name(&m.name);
            format!("            {mod_name}: {mod_name}::{delegate}::new(db.clone()),")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let root = format!(
        r#"//! Client ForgeDB généré — `client.user.find_many().await?`

{mod_decls}
use forgedb_runtime::ForgeDb;

pub struct ForgeDbClient {{
    inner: ForgeDb,
{delegate_fields}}}

impl ForgeDbClient {{
    pub fn new(db: ForgeDb) -> Self {{
        Self {{
            inner: db.clone(),
{delegate_inits}
        }}
    }}

    pub fn db(&self) -> &ForgeDb {{
        &self.inner
    }}
}}
"#,
        mod_decls = mod_decls,
        delegate_fields = delegate_fields,
        delegate_inits = delegate_inits,
    );

    GeneratedClient { root, modules }
}

fn rust_string_literal(content: &str) -> String {
    format!(
        "\"{}\"",
        content.replace('\\', "\\\\").replace('\"', "\\\"")
    )
}

fn scalar_rust_type(field: &forgedb_schema::Field) -> String {
    match &field.kind {
        FieldKind::Scalar(t) => match t {
            ScalarType::String => "String".into(),
            ScalarType::Int => "i32".into(),
            ScalarType::Float => "f64".into(),
            ScalarType::Boolean => "bool".into(),
            ScalarType::DateTime => "chrono::DateTime<chrono::Utc>".into(),
            ScalarType::Uuid => "uuid::Uuid".into(),
        },
        FieldKind::Model(_) => "String".into(),
    }
}

fn generate_update_apply(model: &forgedb_schema::Model) -> String {
    let mut s = String::new();
    for field in table_columns(model) {
        if field.attributes.iter().any(|a| matches!(a, Attribute::Id)) {
            continue;
        }
        let fname = &field.name;
        s.push_str(&format!(
            "        if let Some(v) = input.{fname} {{ current.{fname} = v; }}\n"
        ));
    }
    s
}

fn generate_update_set_sql(model: &forgedb_schema::Model) -> String {
    let mut parts = Vec::new();
    let mut idx = 2i32;
    for field in table_columns(model) {
        if field.attributes.iter().any(|a| matches!(a, Attribute::Id)) {
            continue;
        }
        parts.push(format!(
            "\"{}\" = ${idx}",
            column_name(&field.name),
        ));
        idx += 1;
    }
    parts.join(", ")
}

fn generate_update_binds(model: &forgedb_schema::Model) -> String {
    let mut s = String::new();
    for field in table_columns(model) {
        if field.attributes.iter().any(|a| matches!(a, Attribute::Id)) {
            continue;
        }
        let fname = &field.name;
        s.push_str(&format!("            .bind(current.{fname})\n"));
    }
    s
}
