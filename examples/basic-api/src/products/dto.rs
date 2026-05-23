use rustforge::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateProductDto {
    #[validate(length(min = 2))]
    pub name: String,

    #[validate(length(min = 1))]
    pub description: String,

    #[validate(range(min = 0.01))]
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
}
