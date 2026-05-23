use std::sync::RwLock;

use uuid::Uuid;

use super::dto::CreateCustomerDto;
use super::entities::Customer;

/// Client de démo (même ID utilisé dans `orders` pour les tests manuels).
pub const DEMO_CUSTOMER_ID: Uuid =
    Uuid::from_bytes([0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44, 0x00, 0x00]);

pub struct CustomerRepository {
    items: RwLock<Vec<Customer>>,
}

impl CustomerRepository {
    fn seed() -> Vec<Customer> {
        vec![Customer {
            id: DEMO_CUSTOMER_ID,
            name: "Demo Shop Client".into(),
            email: "demo@shop.example".into(),
        }]
    }
}

impl Default for CustomerRepository {
    fn default() -> Self {
        Self {
            items: RwLock::new(Self::seed()),
        }
    }
}

impl CustomerRepository {
    pub fn find_all(&self) -> Vec<Customer> {
        self.items.read().unwrap().clone()
    }

    pub fn find_by_id(&self, id: Uuid) -> Option<Customer> {
        self.items
            .read()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned()
    }

    pub fn find_by_email(&self, email: &str) -> Option<Customer> {
        self.items
            .read()
            .unwrap()
            .iter()
            .find(|c| c.email.eq_ignore_ascii_case(email))
            .cloned()
    }

    pub fn create(&self, dto: CreateCustomerDto) -> Customer {
        let customer = Customer {
            id: Uuid::new_v4(),
            name: dto.name,
            email: dto.email,
        };
        self.items.write().unwrap().push(customer.clone());
        customer
    }
}
