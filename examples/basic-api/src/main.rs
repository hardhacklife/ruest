mod app_module;
mod products;
mod users;

use ruest::prelude::*;

use app_module::AppModule;

#[tokio::main]
async fn main() -> Result<(), CoreError> {
    ruest::logger::init();
    ruest::bootstrap_app(AppModule)
        .expect("bootstrap failed")
        .port(3000)
        .listen()
        .await
}
