mod db;
mod generate;
mod new;
mod templates;
mod util;

use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use db::{cmd_db, cmd_generate as cmd_ruestdb_generate, DbCommands, MigrateAction};
use generate::{GenerateTarget, cmd_generate};
use new::{cmd_new, detect_ruest_path};

#[derive(Parser)]
#[command(
    name = "ruest",
    about = "RUEST — Rust + NestJS (backend framework)",
    long_about = "RUEST — Rust + NestJS.\nhttps://github.com/hardhacklife/ruest"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new application (ruest new my-api)
    New {
        name: String,
        /// Template: api, microservice, websocket-chat
        #[arg(short, long)]
        template: Option<String>,
        /// Path to ruest crate (monorepo)
        #[arg(long, default_value = "../ruest")]
        ruest_path: String,
    },
    /// Generate code (alias: ruest g)
    #[command(alias = "g")]
    Generate {
        #[command(subcommand)]
        target: Option<GenerateTargetCli>,
    },
    /// RuestDB — schema, client, migrations (Prisma-like)
    #[command(subcommand)]
    Db(DbCommands),
    /// Migrations RuestDB (`ruest migrate dev`)
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },
    /// Génère le client RuestDB depuis `schema.ruest`
    #[command(name = "generate-client")]
    GenerateClient,
    /// Build the project (incremental dev profile)
    Build,
    /// Run the application
    Start {
        /// Hot reload via cargo-watch (install: cargo install cargo-watch)
        #[arg(long)]
        watch: bool,
    },
    /// Run tests
    Test,
    /// Check project layout and give hints
    Doctor,
}

#[derive(Subcommand)]
enum GenerateTargetCli {
    /// Generate a full REST resource (dto, entity, repo, service, controller, module)
    Resource { name: String },
    Module { name: String },
    Controller { name: String },
    Service { name: String },
}

impl From<GenerateTargetCli> for GenerateTarget {
    fn from(value: GenerateTargetCli) -> Self {
        match value {
            GenerateTargetCli::Resource { name } => GenerateTarget::Resource { name },
            GenerateTargetCli::Module { name } => GenerateTarget::Module { name },
            GenerateTargetCli::Controller { name } => GenerateTarget::Controller { name },
            GenerateTargetCli::Service { name } => GenerateTarget::Service { name },
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New {
            name,
            template,
            ruest_path,
        } => cmd_new(&name, template.as_deref(), &ruest_path),
        Commands::Generate { target } => match target {
            Some(t) => cmd_generate(t.into()),
            None => cmd_ruestdb_generate(),
        },
        Commands::Db(command) => cmd_db(command),
        Commands::Migrate { action } => cmd_db(DbCommands::Migrate { action }),
        Commands::GenerateClient => cmd_ruestdb_generate(),
        Commands::Build => run_cargo(&["build"]),
        Commands::Start { watch } => cmd_start(watch),
        Commands::Test => run_cargo(&["test"]),
        Commands::Doctor => cmd_doctor(),
    }
}

fn cmd_start(watch: bool) -> Result<()> {
    if watch {
        let status = Command::new("cargo")
            .args(["watch", "-x", "run"])
            .status()
            .context("cargo-watch not found — run: cargo install cargo-watch")?;
        if !status.success() {
            bail!("ruest start --watch failed");
        }
    } else {
        run_cargo(&["run"])?;
    }
    Ok(())
}

fn cmd_doctor() -> Result<()> {
    println!("Ruest Doctor — quick checks\n");
    let checks = [
        ("src/main.rs", "entry point"),
        ("src/modules/", "modules directory"),
        ("src/config/", "config directory"),
        ("Cargo.toml", "manifest"),
    ];
    for (path, desc) in checks {
        let ok = std::path::Path::new(path).exists();
        println!("  {} {path} ({desc})", if ok { "✓" } else { "✗" });
    }
    println!("\nTips:");
    println!("  ruest g resource users   — full CRUD scaffold");
    println!("  ruest start --watch      — hot reload");
    println!("  use ruest::prelude::*; — single import");
    db::doctor_db_hints();
    Ok(())
}

fn run_cargo(args: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .context("failed to run cargo")?;
    if !status.success() {
        bail!("cargo {:?} failed", args);
    }
    Ok(())
}
