use ruest::prelude::*;

use crate::products::products_module::ProductsModule;
use crate::users::users_module::UsersModule;

#[module(imports = [UsersModule, ProductsModule])]
pub struct AppModule;
