use rustforge::prelude::*;

use super::users_controller::UserController;
use super::users_service::UserService;

#[module(
    controllers = [UserController],
    providers = [UserService]
)]
pub struct UsersModule;
