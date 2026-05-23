use std::sync::RwLock;

use uuid::Uuid;

use super::dto::CreateOrderDto;
use super::entities::{Order, OrderStatus};

#[derive(Default)]
pub struct OrderRepository {
    items: RwLock<Vec<Order>>,
}

impl OrderRepository {
    pub fn find_all(&self) -> Vec<Order> {
        self.items.read().unwrap().clone()
    }

    pub fn create(&self, dto: CreateOrderDto) -> Order {
        let order = Order {
            id: Uuid::new_v4(),
            customer_id: dto.customer_id,
            product_name: dto.product_name,
            amount: dto.amount,
            status: OrderStatus::Pending,
        };
        self.items.write().unwrap().push(order.clone());
        order
    }
}
