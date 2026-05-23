use std::env;
use std::path::PathBuf;

use anyhow::Result;
use forgedb_migrate::{
    create_migration, db_init, generate_client, migrate_apply, migrate_reset, MigrateError,
};

#[derive(clap::Subcommand)]
pub enum DbCommands {
    /// Initialise `schema.forge` et `forgedb/migrations/`
    Init,
    /// Génère le client Rust (`generated/forgedb/`)
    Generate,
    /// Crée une migration SQL depuis le schema
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },
}

#[derive(clap::Subcommand)]
pub enum MigrateAction {
    /// Crée + applique une migration (`init` par défaut)
    Dev {
        #[arg(long, default_value = "init")]
        name: String,
    },
    /// Applique les migrations en attente
    Deploy,
    /// Drop tables + réapplique (dev only)
    Reset,
}

pub fn cmd_db(command: DbCommands) -> Result<()> {
    let root = project_root();
    match command {
        DbCommands::Init => db_init(&root).map_err(map_err),
        DbCommands::Generate => generate_client(&root).map_err(map_err),
        DbCommands::Migrate { action } => {
            let rt = tokio::runtime::Runtime::new()?;
            match action {
                MigrateAction::Dev { name } => {
                    create_migration(&root, &name).map_err(map_err)?;
                    rt.block_on(migrate_apply(&root)).map_err(map_err)?;
                }
                MigrateAction::Deploy => {
                    rt.block_on(migrate_apply(&root)).map_err(map_err)?;
                }
                MigrateAction::Reset => {
                    rt.block_on(migrate_reset(&root)).map_err(map_err)?;
                }
            }
            Ok(())
        }
    }
}

fn project_root() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn map_err(e: MigrateError) -> anyhow::Error {
    anyhow::Error::new(e)
}

/// `forge generate` — alias de `forge db generate`
pub fn cmd_generate() -> Result<()> {
    generate_client(&project_root()).map_err(map_err)
}

pub fn doctor_db_hints() {
    println!("  forge db init           — schema.forge + migrations/");
    println!("  forge generate          — client Rust type-safe");
    println!("  forge migrate dev       — migration + apply");
    println!("  DATABASE_URL=postgres://…  — connexion PostgreSQL");
}
