use ruest::prelude::*;

use crate::modules::auth::AuthModule;
use crate::modules::customers::CustomersModule;
use crate::modules::orders::OrdersModule;

/// Module racine : compose les modules métier (pas besoin de lister controllers/services ici).
#[module(imports = [AuthModule, CustomersModule, OrdersModule])]
pub struct AppModule;
