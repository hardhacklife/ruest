//! Shop API — second exemple RUEST (structure README + multi-modules).

mod app_module;
mod common;
mod config;
mod modules;

use ruest::prelude::*;

use app_module::AppModule;

#[tokio::main]
async fn main() -> Result<(), CoreError> {
    ruest::logger::init();

    let security = SecurityConfig::dev();

    bootstrap_app(AppModule)?
        .with_jwt_auth(security)?
        .port(config::port())
        .listen()
        .await
}
