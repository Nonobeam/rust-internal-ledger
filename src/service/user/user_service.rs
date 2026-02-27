use std::sync::Arc;

use crate::common::logger::app_error::AppError;
use crate::domain::user::user::User;
use crate::repository::user::pg_user_repository::PgUserRepository;
use crate::domain::user::user::CreateUserDTO;
use crate::repository::user::user_repository::UserRepository;

pub struct UserService {
    repo: Arc<PgUserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<PgUserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, user: &CreateUserDTO) -> Result<User, AppError> {
        self.repo
            .create(user)
            .await?
            .ok_or(AppError::NotFound(user.name.clone()))
    }
}
