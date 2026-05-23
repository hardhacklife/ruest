use rustforge::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Validate, serde::Deserialize)]
pub struct CreateOrderDto {
    pub customer_id: Uuid,

    #[validate(length(min = 1))]
    pub product_name: String,

    /// Montant TTC (validation manuelle si besoin ; `range` sur f64 limité avec validator 0.18).
    pub amount: f64,
}
