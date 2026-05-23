mod generate;
mod new;
mod templates;
mod util;

use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use generate::{GenerateTarget, cmd_generate};
use new::{cmd_new, detect_rustforge_path};

#[derive(Parser)]
#[command(
    name = "forge",
    about = "Forge — Rust backend framework (NestJS DX + Rust performance)",
    long_about = "Simple, fast, enjoyable Rust backend development.\nhttps://github.com/rustforge/rustforge"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new application (forge new my-api)
    New {
        name: String,
        /// Template: api, microservice, websocket-chat
        #[arg(short, long)]
        template: Option<String>,
        /// Path to rustforge crate (monorepo)
        #[arg(long, default_value = "../rustforge")]
        rustforge_path: String,
    },
    /// Generate code (alias: forge g)
    #[command(alias = "g")]
    Generate {
        #[command(subcommand)]
        target: GenerateTargetCli,
    },
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
            rustforge_path,
        } => cmd_new(&name, template.as_deref(), &rustforge_path),
        Commands::Generate { target } => cmd_generate(target.into()),
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
            bail!("forge start --watch failed");
        }
    } else {
        run_cargo(&["run"])?;
    }
    Ok(())
}

fn cmd_doctor() -> Result<()> {
    println!("Forge Doctor — quick checks\n");
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
    println!("  forge g resource users   — full CRUD scaffold");
    println!("  forge start --watch      — hot reload");
    println!("  use rustforge::prelude::*; — single import");
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
