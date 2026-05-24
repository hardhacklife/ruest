use ruest::prelude::*;

use super::products_controller::ProductController;
use super::products_service::ProductService;

#[module(
    controllers = [ProductController],
    providers = [ProductService]
)]
pub struct ProductsModule;
