use ruest::service;

use super::dto::CreateOrderDto;
use super::entities::Order;
use super::orders_repository::OrderRepository;

#[service]
pub struct OrderService {
    repo: OrderRepository,
}

impl Default for OrderService {
    fn default() -> Self {
        Self {
            repo: OrderRepository::default(),
        }
    }
}

impl OrderService {
    pub async fn find_all(&self) -> Vec<Order> {
        self.repo.find_all()
    }

    pub async fn create(&self, dto: CreateOrderDto) -> Order {
        self.repo.create(dto)
    }
}
