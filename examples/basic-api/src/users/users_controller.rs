use rustforge::prelude::*;

use super::dto::{CreateUserDto, User};
use super::users_service::UserService;

#[controller("/users")]
pub struct UserController {
    service: Inject<UserService>,
}

#[routes]
impl UserController {
    #[get("/")]
    async fn get_users(&self) -> Json<Vec<User>> {
        Json(self.service.find_all().await)
    }

    #[post("/")]
    async fn create_user(&self) -> Json<User> {
        let dto = CreateUserDto {
            email: "new@rustforge.dev".into(),
            name: "New User".into(),
        };
        Json(self.service.create(dto).await)
    }
}
