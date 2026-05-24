use ruest::prelude::*;

use super::AuthController;

#[module(controllers = [AuthController], providers = [JwtDevProvider])]
pub struct AuthModule;
