use ruest::prelude::*;

use super::dto::{CreateUserDto, User};
use super::users_service::UserService;

#[controller("/users")]
pub struct UserController {
    service: Inject<UserService>,
}

#[routes]
impl UserController {
    #[get("/")]
    async fn get_users(&self) -> AppResult<Json<Vec<User>>> {
        Ok(Json(self.service.find_all().await))
    }

    #[post("/")]
    async fn create_user(&self) -> AppResult<Json<User>> {
        let dto = CreateUserDto {
            email: "new@ruest.dev".into(),
            name: "New User".into(),
        };
        Ok(Json(self.service.create(dto).await))
    }
}
