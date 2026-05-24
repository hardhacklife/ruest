use ruest::prelude::*;

#[derive(Debug, Clone, Validate, serde::Deserialize)]
pub struct CreateCustomerDto {
    #[validate(length(min = 2))]
    pub name: String,

    #[validate(email)]
    pub email: String,
}
