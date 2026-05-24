use std::fs;
use std::path::Path;

use anyhow::{bail, Result};

use crate::templates;

#[derive(Debug, Clone, Copy, Default)]
pub enum AppTemplate {
    #[default]
    Api,
    Microservice,
    WebsocketChat,
}

impl AppTemplate {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "api" | "default" => Ok(Self::Api),
            "microservice" => Ok(Self::Microservice),
            "websocket-chat" | "ws" => Ok(Self::WebsocketChat),
            other => bail!("unknown template `{other}`; try: api, microservice, websocket-chat"),
        }
    }
}

pub fn cmd_new(name: &str, template: Option<&str>, ruest_path: &str) -> Result<()> {
    let template = match template {
        Some(t) => AppTemplate::from_str(t)?,
        None => AppTemplate::default(),
    };

    let dir = Path::new(name);
    if dir.exists() {
        bail!("directory `{name}` already exists");
    }

    fs::create_dir_all(dir.join("src/modules/app"))?;
    fs::create_dir_all(dir.join("src/common"))?;
    fs::create_dir_all(dir.join("src/config"))?;

    fs::write(dir.join("Cargo.toml"), templates::app_cargo(name, ruest_path))?;
    fs::write(dir.join("src/main.rs"), templates::app_main("app"))?;
    fs::write(dir.join("src/config/mod.rs"), templates::app_config())?;
    fs::write(dir.join("src/modules/mod.rs"), templates::app_modules_mod())?;
    fs::write(dir.join("src/modules/app/mod.rs"), "pub mod app_module;\n")?;
    fs::write(
        dir.join("src/modules/app/app_module.rs"),
        templates::app_module(),
    )?;
    fs::write(
        dir.join("src/common/mod.rs"),
        "//! Helpers partagés (filtres, guards, utils).\n",
    )?;
    fs::write(
        dir.join(".env.example"),
        "PORT=3000\nRUST_LOG=info\nDATABASE_URL=postgres://localhost/app\n",
    )?;
    fs::write(
        dir.join("README.md"),
        format!(
            "# {name}\n\n```bash\nruest start          # lancer\nruest start --watch  # hot reload (cargo-watch)\nruest g resource users\n```\n"
        ),
    )?;

    match template {
        AppTemplate::Api => {}
        AppTemplate::Microservice => {
            fs::write(
                dir.join("src/config/mod.rs"),
                r#"pub fn port() -> u16 {
    std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(4000)
}
"#,
            )?;
        }
        AppTemplate::WebsocketChat => {
            fs::write(
                dir.join("src/modules/app/app_module.rs"),
                r#"use ruest::prelude::*;

#[module(controllers = [], providers = [])]
pub struct AppModule;

// Ajoutez un gateway WebSocket : ruest g resource chat
"#,
            )?;
        }
    }

    let template_name = match template {
        AppTemplate::Api => "api",
        AppTemplate::Microservice => "microservice",
        AppTemplate::WebsocketChat => "websocket-chat",
    };
    println!("✓ Created RUEST app `{name}` (template: {template_name})");
    println!("  cd {name}");
    println!("  ruest start");
    Ok(())
}

pub fn detect_ruest_path() -> String {
    if Path::new("ruest").exists() {
        "../ruest".into()
    } else {
        "../ruest".into()
    }
}
