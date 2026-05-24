use ruest::prelude::*;

use crate::modules::customers::customers_repository::DEMO_CUSTOMER_ID;
use crate::modules::customers::CustomerService;

use super::dto::CreateOrderDto;
use super::entities::Order;
use super::orders_service::OrderService;

#[controller("/orders")]
pub struct OrderController {
    orders: Inject<OrderService>,
    customers: Inject<CustomerService>,
}

#[routes]
impl OrderController {
    #[get("/")]
    async fn list(&self) -> AppResult<Json<Vec<Order>>> {
        Ok(Json(self.orders.find_all().await))
    }

    #[post("/")]
    async fn create(&self) -> AppResult<Json<Order>> {
        // En production : ValidatedJson<CreateOrderDto> (extracteurs Phase 2)
        let dto = CreateOrderDto {
            customer_id: DEMO_CUSTOMER_ID,
            product_name: "RUEST Hoodie".into(),
            amount: 49.90,
        };

        if dto.validate().is_err() {
            return Err(ruest_err!(BadRequest, "Invalid order data"));
        }

        if self.customers.find_by_id(dto.customer_id).await.is_none() {
            return Err(ruest_err!(
                BadRequest,
                "Customer not found — create a customer first (POST /customers/)"
            ));
        }

        Ok(Json(self.orders.create(dto).await))
    }
}
