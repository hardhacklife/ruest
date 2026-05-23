use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}
