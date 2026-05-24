use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::templates;
use crate::util::{to_title_case, to_type_name};

pub enum GenerateTarget {
    Module { name: String },
    Controller { name: String },
    Service { name: String },
    Resource { name: String },
}

pub fn cmd_generate(target: GenerateTarget) -> Result<()> {
    match target {
        GenerateTarget::Resource { name } => generate_resource(&name),
        GenerateTarget::Module { name } => generate_module(&name),
        GenerateTarget::Controller { name } => generate_controller(&name),
        GenerateTarget::Service { name } => generate_service(&name),
    }
}

fn generate_resource(name: &str) -> Result<()> {
    let title = to_type_name(name);
    let base = Path::new("src/modules").join(name);

    fs::create_dir_all(base.join("dto"))?;
    fs::create_dir_all(base.join("entities"))?;

    fs::write(base.join("mod.rs"), templates::resource_mod(name, &title))?;
    fs::write(base.join("dto/mod.rs"), templates::resource_dto(&title))?;
    fs::write(
        base.join("entities/mod.rs"),
        templates::resource_entity(&title),
    )?;
    fs::write(
        base.join(format!("{name}_service.rs")),
        templates::resource_service(name, &title),
    )?;
    fs::write(
        base.join(format!("{name}_repository.rs")),
        templates::resource_repository(&title),
    )?;
    fs::write(
        base.join(format!("{name}_controller.rs")),
        templates::resource_controller(name, &title),
    )?;
    fs::write(
        base.join(format!("{name}_module.rs")),
        templates::resource_module(name, &title),
    )?;

    patch_app_module(name, &title)?;

    println!("✓ Generated resource `{name}`");
    println!("  src/modules/{name}/");
    println!("    dto/ entities/ {name}.controller.rs {name}.service.rs");
    println!("    {name}.repository.rs {name}.module.rs");
    println!("\n  Register in src/modules/mod.rs: pub mod {name};");
    println!("  Wire in AppModule if using convention (see docs/DX.md)");
    Ok(())
}

fn patch_app_module(name: &str, title: &str) -> Result<()> {
    let app_module = Path::new("src/modules/app/app_module.rs");
    if !app_module.exists() {
        return Ok(());
    }
    let hint = format!(
        "\n// TODO: import and register {title}Module — providers/controllers auto via #[module]\n// pub use crate::modules::{name}::{name}_module::{title}Module;\n"
    );
    let mut content = fs::read_to_string(app_module)?;
    if !content.contains(&format!("{title}Module")) {
        content.push_str(&hint);
        fs::write(app_module, content)?;
    }
    Ok(())
}

fn generate_module(name: &str) -> Result<()> {
    let title = to_type_name(name);
    let base = Path::new("src/modules").join(name);
    fs::create_dir_all(&base)?;
    fs::write(
        base.join("mod.rs"),
        format!("pub mod {name}_module;\n"),
    )?;
    fs::write(
        base.join(format!("{name}_module.rs")),
        format!(
            "use ruest::prelude::*;\n\n#[module(controllers = [], providers = [])]\npub struct {title}Module;\n"
        ),
    )?;
    println!("✓ Generated module `{name}`");
    Ok(())
}

fn generate_controller(name: &str) -> Result<()> {
    let title = to_type_name(name);
    let path = Path::new("src/modules")
        .join(name)
        .join(format!("{name}_controller.rs"));
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(
        &path,
        format!(
            r#"use ruest::prelude::*;

#[controller("/{name}")]
pub struct {title}Controller {{
    service: Inject<{title}Service>,
}}

#[routes]
impl {title}Controller {{
    #[get("/")]
    async fn list(&self) -> AppResult<Json<Vec<String>>> {{
        Ok(Json(vec![]))
    }}
}}
"#
        ),
    )?;
    println!("✓ Generated controller at {}", path.display());
    Ok(())
}

fn generate_service(name: &str) -> Result<()> {
    let title = to_type_name(name);
    let path = Path::new("src/modules")
        .join(name)
        .join(format!("{name}_service.rs"));
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(
        &path,
        format!(
            r#"use ruest::prelude::*;

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
    println!("✓ Generated service at {}", path.display());
    Ok(())
}
