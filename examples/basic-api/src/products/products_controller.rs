use rustforge::prelude::*;

use super::dto::{CreateProductDto, Product};
use super::products_service::ProductService;

#[controller("/products")]
pub struct ProductController {
    service: Inject<ProductService>,
}

#[routes]
impl ProductController {
    #[get("/")]
    async fn get_products(&self) -> AppResult<Json<Vec<Product>>> {
        Ok(Json(self.service.find_all().await))
    }

    #[post("/")]
    async fn create_product(&self) -> AppResult<Json<Product>> {
        let dto = CreateProductDto {
            name: "New Product".into(),
            description: "Created via basic-api".into(),
            price: 19.99,
        };
        Ok(Json(self.service.create(dto).await))
    }
}
