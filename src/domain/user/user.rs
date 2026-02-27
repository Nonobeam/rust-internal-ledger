use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: String,
    pub name: String,
    pub user_type: Uuid,
    pub created_at: DateTime<Utc>,
}

pub struct CreateUserDTO {
    pub name: String,
    pub user_type: Uuid,
}

impl CreateUserDTO {
    pub fn new(name: String, user_type: Uuid) -> Self {
        Self {
            name,
            user_type,
        }
    }
}