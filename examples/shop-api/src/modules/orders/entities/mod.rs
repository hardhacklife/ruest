use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub product_name: String,
    pub amount: f64,
    pub status: OrderStatus,
}
