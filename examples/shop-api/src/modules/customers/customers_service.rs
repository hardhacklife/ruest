use rustforge::service;

use uuid::Uuid;

use super::customers_repository::CustomerRepository;
use super::dto::CreateCustomerDto;
use super::entities::Customer;

#[service]
pub struct CustomerService {
    repo: CustomerRepository,
}

impl Default for CustomerService {
    fn default() -> Self {
        Self {
            repo: CustomerRepository::default(),
        }
    }
}

impl CustomerService {
    pub async fn find_all(&self) -> Vec<Customer> {
        self.repo.find_all()
    }

    pub async fn find_by_id(&self, id: Uuid) -> Option<Customer> {
        self.repo.find_by_id(id)
    }

    pub async fn email_exists(&self, email: &str) -> bool {
        self.repo.find_by_email(email).is_some()
    }

    pub async fn create(&self, dto: CreateCustomerDto) -> Customer {
        self.repo.create(dto)
    }
}
