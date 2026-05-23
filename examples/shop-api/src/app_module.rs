use rustforge::prelude::*;

use crate::modules::customers::CustomersModule;
use crate::modules::orders::OrdersModule;

/// Module racine : compose les modules métier (pas besoin de lister controllers/services ici).
#[module(imports = [CustomersModule, OrdersModule])]
pub struct AppModule;
