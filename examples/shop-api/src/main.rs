//! Shop API — second exemple RustForge (structure README + multi-modules).

mod app_module;
mod common;
mod config;
mod modules;

use rustforge::prelude::*;

use app_module::AppModule;

#[tokio::main]
async fn main() -> Result<(), CoreError> {
    rustforge::logger::init();

    bootstrap_app(AppModule)?
        .port(config::port())
        .listen()
        .await
}
