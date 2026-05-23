use std::sync::RwLock;

use rustforge::service;
use uuid::Uuid;

use super::dto::{CreateProductDto, Product};

#[service]
pub struct ProductService {
    products: RwLock<Vec<Product>>,
}

impl Default for ProductService {
    fn default() -> Self {
        Self {
            products: RwLock::new(vec![Product {
                id: Uuid::new_v4(),
                name: "Demo Product".into(),
                description: "Sample product for RustForge basic-api".into(),
                price: 9.99,
            }]),
        }
    }
}

impl ProductService {
    pub async fn find_all(&self) -> Vec<Product> {
        self.products.read().unwrap().clone()
    }

    pub async fn create(&self, dto: CreateProductDto) -> Product {
        let product = Product {
            id: Uuid::new_v4(),
            name: dto.name,
            description: dto.description,
            price: dto.price,
        };
        self.products.write().unwrap().push(product.clone());
        product
    }
}
