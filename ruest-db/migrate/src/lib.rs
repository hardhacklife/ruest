//! Exécution des migrations RuestDB (`prisma/migrations` style → `ruestdb/migrations`).

use std::fs;
use std::path::{Path, PathBuf};

use ruest_db_runtime::RuestDb;
use sqlx::Executor;
use thiserror::Error;

pub const MIGRATIONS_DIR: &str = "ruestdb/migrations";
pub const SCHEMA_FILE: &str = "schema.ruest";

#[derive(Debug, Error)]
pub enum MigrateError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("parse error: {0}")]
    Parse(#[from] ruest_db_parser::ParseError),

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("{0}")]
    Message(String),
}

/// Crée `schema.ruest` et le dossier migrations (projet neuf).
pub fn db_init(project_root: &Path) -> Result<(), MigrateError> {
    let schema_path = project_root.join(SCHEMA_FILE);
    if !schema_path.exists() {
        fs::write(&schema_path, DEFAULT_SCHEMA)?;
        println!("Created {}", schema_path.display());
    }

    let migrations = project_root.join(MIGRATIONS_DIR);
    fs::create_dir_all(&migrations)?;
    println!("Created {}", migrations.display());
    Ok(())
}

/// Génère `generated/ruestdb/` (client Rust type-safe).
pub fn generate_client(project_root: &Path) -> Result<(), MigrateError> {
    let schema_src = fs::read_to_string(project_root.join(SCHEMA_FILE))?;
    let schema = ruest_db_parser::parse_schema(&schema_src)?;
    let generated = ruest_db_codegen::generate_client(&schema);

    let out = project_root.join("generated/ruestdb");
    fs::create_dir_all(&out)?;
    fs::write(out.join("mod.rs"), generated.root)?;

    for (name, src) in generated.modules {
        fs::write(out.join(format!("{name}.rs")), src)?;
    }

    println!("Generated RuestDB client in {}", out.display());
    Ok(())
}

/// Génère une migration SQL depuis `schema.ruest`.
pub fn create_migration(project_root: &Path, name: &str) -> Result<PathBuf, MigrateError> {
    let schema_src = fs::read_to_string(project_root.join(SCHEMA_FILE))?;
    let schema = ruest_db_parser::parse_schema(&schema_src)?;
    let sql = ruest_db_codegen::generate_migration_sql(&schema);

    let stamp = chrono_lite_timestamp();
    let dir = project_root.join(MIGRATIONS_DIR).join(format!("{stamp}_{name}"));
    fs::create_dir_all(&dir)?;
    let file = dir.join("migration.sql");
    fs::write(&file, sql)?;
    println!("Created migration {}", dir.display());
    Ok(dir)
}

/// Applique les migrations en attente (`ruest migrate dev` / `deploy`).
pub async fn migrate_apply(project_root: &Path) -> Result<(), MigrateError> {
    let db = RuestDb::connect_from_env()
        .await
        .map_err(|e| MigrateError::Message(e.to_string()))?;

    ensure_migrations_table(db.pool()).await?;

    let applied = applied_migrations(db.pool()).await?;
    let mut pending = list_migrations(project_root)?;
    pending.sort();

    for dir in pending {
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| MigrateError::Message("invalid migration dir".into()))?;
        if applied.iter().any(|a| a == name) {
            continue;
        }
        let sql_path = dir.join("migration.sql");
        let sql = fs::read_to_string(&sql_path)?;
        tracing::info!(migration = name, "applying");
        db.pool().execute(sql.as_str()).await?;
        sqlx::query("INSERT INTO _ruestdb_migrations (name) VALUES ($1)")
            .bind(name)
            .execute(db.pool())
            .await?;
        println!("Applied {name}");
    }

    Ok(())
}

/// Supprime les tables et réapplique (dangereux — dev uniquement).
pub async fn migrate_reset(project_root: &Path) -> Result<(), MigrateError> {
    let db = RuestDb::connect_from_env()
        .await
        .map_err(|e| MigrateError::Message(e.to_string()))?;

    let schema_src = fs::read_to_string(project_root.join(SCHEMA_FILE))?;
    let schema = ruest_db_parser::parse_schema(&schema_src)?;

    for model in schema.models.iter().rev() {
        let table = ruest_db_codegen::table_name(&model.name);
        let sql = format!("DROP TABLE IF EXISTS \"{table}\" CASCADE");
        db.pool().execute(sql.as_str()).await.ok();
    }

    sqlx::query("DROP TABLE IF EXISTS _ruestdb_migrations CASCADE")
        .execute(db.pool())
        .await?;

    create_migration(project_root, "init")?;
    migrate_apply(project_root).await
}

async fn ensure_migrations_table(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _ruestdb_migrations (
            name TEXT PRIMARY KEY,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn applied_migrations(pool: &sqlx::PgPool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query_scalar::<_, String>("SELECT name FROM _ruestdb_migrations ORDER BY name")
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

fn list_migrations(project_root: &Path) -> Result<Vec<PathBuf>, MigrateError> {
    let dir = project_root.join(MIGRATIONS_DIR);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            out.push(entry.path());
        }
    }
    Ok(out)
}

fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{secs}")
}

const DEFAULT_SCHEMA: &str = r#"// RuestDB schema — https://github.com/hardhacklife/ruest
model User {
  id    String @id @default(uuid())
  email String @unique
  name  String
}
"#;
