use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustforge", about = "RustForge CLI — backend framework for Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new RustForge application
    New {
        name: String,
    },
    /// Generate code scaffolding
    Generate {
        #[command(subcommand)]
        target: GenerateTarget,
    },
    /// Build the current project
    Build,
    /// Start the application (cargo run)
    Start,
    /// Run tests
    Test,
}

#[derive(Subcommand)]
enum GenerateTarget {
    Module { name: String },
    Controller { name: String },
    Service { name: String },
    Resource { name: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => cmd_new(&name),
        Commands::Generate { target } => cmd_generate(target),
        Commands::Build => run_cargo(&["build"]),
        Commands::Start => run_cargo(&["run"]),
        Commands::Test => run_cargo(&["test"]),
    }
}

fn cmd_new(name: &str) -> Result<()> {
    let dir = Path::new(name);
    if dir.exists() {
        bail!("directory `{name}` already exists");
    }

    fs::create_dir_all(dir.join("src/modules"))?;

    fs::write(
        dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
rustforge = {{ path = "../rustforge" }}
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1", features = ["derive"] }}
validator = {{ version = "0.18", features = ["derive"] }}
"#
        ),
    )?;

    fs::write(
        dir.join("src/main.rs"),
        r#"mod app_module;

use rustforge::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CoreError> {
    rustforge::logger::init();
    RustForgeFactory::bootstrap(app_module::AppModule)
        .expect("bootstrap")
        .port(3000)
        .listen()
        .await
}
"#,
    )?;

    fs::write(
        dir.join("src/app_module.rs"),
        r#"use rustforge::prelude::*;

#[module(controllers = [], providers = [])]
pub struct AppModule;
"#,
    )?;

    fs::write(dir.join(".env.example"), "PORT=3000\nDATABASE_URL=\n")?;

    println!("Created RustForge app `{name}`");
    println!("  cd {name} && cargo run");
    Ok(())
}

fn cmd_generate(target: GenerateTarget) -> Result<()> {
    match target {
        GenerateTarget::Module { name } => {
            let module_dir = Path::new("src/modules").join(&name);
            fs::create_dir_all(&module_dir)?;
            let title = to_title_case(&name);
            fs::write(
                module_dir.join(format!("{name}.module.rs")),
                format!(
                    "use rustforge::prelude::*;\n\n#[module(controllers = [{title}Controller], providers = [{title}Service])]\npub struct {title}Module;\n"
                ),
            )?;
            println!("Generated module `{name}`");
        }
        GenerateTarget::Controller { name } => {
            let title = to_title_case(&name);
            fs::write(
                Path::new("src").join(format!("{name}.controller.rs")),
                format!(
                    r#"use rustforge::prelude::*;

#[controller("/{name}")]
pub struct {title}Controller {{
    service: Inject<{title}Service>,
}}

#[routes]
impl {title}Controller {{
    #[get("/")]
    async fn list(&self) -> Json<Vec<String>> {{
        Json(self.service.list().await)
    }}
}}
"#
                ),
            )?;
            println!("Generated controller `{name}`");
        }
        GenerateTarget::Service { name } => {
            let title = to_title_case(&name);
            fs::write(
                Path::new("src").join(format!("{name}.service.rs")),
                format!(
                    r#"use rustforge::prelude::*;

#[service]
#[derive(Default)]
pub struct {title}Service;

impl {title}Service {{
    pub async fn list(&self) -> Vec<String> {{
        vec![]
    }}
}}
"#
                ),
            )?;
            println!("Generated service `{name}`");
        }
        GenerateTarget::Resource { name } => {
            cmd_generate(GenerateTarget::Service {
                name: name.clone(),
            })?;
            cmd_generate(GenerateTarget::Controller { name: name.clone() })?;
            cmd_generate(GenerateTarget::Module { name })?;
        }
    }
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

fn to_title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
