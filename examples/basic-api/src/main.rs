mod users;

use rustforge::prelude::*;

use users::users_module::UsersModule;

#[tokio::main]
async fn main() -> Result<(), CoreError> {
    rustforge::logger::init();
    rustforge::bootstrap_app(UsersModule)
        .expect("bootstrap failed")
        .port(3000)
        .listen()
        .await
}
