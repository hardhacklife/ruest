use std::sync::RwLock;

use ruest::service;
use uuid::Uuid;

use super::dto::{CreateUserDto, User};

#[service]
pub struct UserService {
    users: RwLock<Vec<User>>,
}

impl Default for UserService {
    fn default() -> Self {
        Self {
            users: RwLock::new(vec![User {
                id: Uuid::new_v4(),
                email: "demo@ruest.dev".into(),
                name: "Demo User".into(),
            }]),
        }
    }
}

impl UserService {
    pub async fn find_all(&self) -> Vec<User> {
        self.users.read().unwrap().clone()
    }

    pub async fn create(&self, dto: CreateUserDto) -> User {
        let user = User {
            id: Uuid::new_v4(),
            email: dto.email,
            name: dto.name,
        };
        self.users.write().unwrap().push(user.clone());
        user
    }
}
