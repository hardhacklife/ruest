//! Templates de génération de code (convention over configuration).

pub fn app_cargo(name: &str, ruest_path: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
ruest-framework = {{ path = "{ruest_path}" }}
tokio = {{ version = "1", features = ["macros", "rt-multi-thread"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
validator = {{ version = "0.18", features = ["derive"] }}
uuid = {{ version = "1", features = ["v4", "serde"] }}

[profile.dev]
incremental = true
"#
    )
}

pub fn app_main(root_module: &str) -> String {
    format!(
        r#"//! Point d'entrée — une seule ligne bootstrap.

mod config;
mod modules;

use ruest::prelude::*;

use modules::app::app_module::AppModule;

#[tokio::main]
async fn main() -> AppResult<()> {{
    ruest::logger::init();
    bootstrap_app(AppModule)?
        .port(config::port())
        .listen()
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
    Ok(())
}}
"#
    )
}

pub fn app_config() -> &'static str {
    r#"//! Configuration (convention : variables d'environnement).

pub fn port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000)
}
"#
}

pub fn app_modules_mod() -> &'static str {
    r#"//! Modules métier (un dossier par domaine).

pub mod app;
"#
}

pub fn app_module() -> &'static str {
    r#"use ruest::prelude::*;

/// Module racine — ajoutez vos modules enfants dans `wire_routes` si besoin.
#[module(controllers = [], providers = [])]
pub struct AppModule;
"#
}

pub fn resource_mod(name: &str, title: &str) -> String {
    format!(
        r#"pub mod dto;
pub mod entities;
pub mod {name}_controller;
pub mod {name}_repository;
pub mod {name}_service;
pub mod {name}_module;
"#
    )
}

pub fn resource_module(name: &str, title: &str) -> String {
    format!(
        r#"use ruest::prelude::*;

use super::{name}_controller::{title}Controller;
use super::{name}_service::{title}Service;

#[module(controllers = [{title}Controller], providers = [{title}Service])]
pub struct {title}Module;
"#
    )
}

pub fn resource_dto(title: &str) -> String {
    format!(
        r#"use ruest::prelude::*;

#[derive(Debug, Clone, Validate, serde::Serialize, serde::Deserialize)]
pub struct Create{title}Dto {{
    #[validate(length(min = 2))]
    pub name: String,

    #[validate(email)]
    pub email: String,
}}
"#
    )
}

pub fn resource_entity(title: &str) -> String {
    format!(
        r#"use ruest::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct {title} {{
    pub id: Uuid,
    pub name: String,
    pub email: String,
}}
"#
    )
}

pub fn resource_service(name: &str, title: &str) -> String {
    format!(
        r#"use std::sync::RwLock;

use super::dto::Create{title}Dto;
use super::entities::{title};
use super::{name}_repository::{title}Repository;

#[service]
pub struct {title}Service {{
    repo: {title}Repository,
}}

impl Default for {title}Service {{
    fn default() -> Self {{
        Self {{
            repo: {title}Repository::default(),
        }}
    }}
}}

impl {title}Service {{
    pub async fn find_all(&self) -> Vec<{title}> {{
        self.repo.find_all()
    }}

    pub async fn create(&self, dto: Create{title}Dto) -> {title} {{
        self.repo.create(dto)
    }}
}}
"#
    )
}

pub fn resource_repository(title: &str) -> String {
    format!(
        r#"use std::sync::RwLock;

use uuid::Uuid;

use super::dto::Create{title}Dto;
use super::entities::{title};

#[derive(Default)]
pub struct {title}Repository {{
    items: RwLock<Vec<{title}>>,
}}

impl {title}Repository {{
    pub fn find_all(&self) -> Vec<{title}> {{
        self.items.read().unwrap().clone()
    }}

    pub fn create(&self, dto: Create{title}Dto) -> {title} {{
        let item = {title} {{
            id: Uuid::new_v4(),
            name: dto.name,
            email: dto.email,
        }};
        self.items.write().unwrap().push(item.clone());
        item
    }}
}}
"#
    )
}

pub fn resource_controller(name: &str, title: &str) -> String {
    format!(
        r#"use ruest::prelude::*;

use super::dto::Create{title}Dto;
use super::entities::{title};
use super::{name}_service::{title}Service;

#[controller("/{name}")]
pub struct {title}Controller {{
    service: Inject<{title}Service>,
}}

#[routes]
impl {title}Controller {{
    #[get("/")]
    async fn list(&self) -> AppResult<Json<Vec<{title}>>> {{
        Ok(Json(self.service.find_all().await))
    }}

    #[post("/")]
    async fn create(&self) -> AppResult<Json<{title}>> {{
        let dto = Create{title}Dto {{
            name: "New".into(),
            email: "new@example.com".into(),
        }};
        Ok(Json(self.service.create(dto).await))
    }}
}}
"#
    )
}
