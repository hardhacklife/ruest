use ruest::prelude::*;

use super::customers_service::CustomerService;
use super::dto::CreateCustomerDto;
use super::entities::Customer;

#[controller("/customers")]
pub struct CustomerController {
    service: Inject<CustomerService>,
}

#[routes]
impl CustomerController {
    #[get("/")]
    async fn list(&self) -> AppResult<Json<Vec<Customer>>> {
        Ok(Json(self.service.find_all().await))
    }

    #[post("/")]
    async fn create(&self) -> AppResult<Json<Customer>> {
        let dto = CreateCustomerDto {
            name: "Alice Martin".into(),
            email: "alice@shop.example".into(),
        };

        if !dto.validate().is_ok() {
            return Err(ruest_err!(BadRequest, "Invalid customer data"));
        }

        if self.service.email_exists(&dto.email).await {
            return Err(ruest_err!(Conflict, "Email already exists"));
        }

        Ok(Json(self.service.create(dto).await))
    }
}
