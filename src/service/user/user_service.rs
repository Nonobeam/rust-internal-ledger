use std::sync::Arc;

use crate::common::logger::app_error::AppError;
use crate::domain::user::user::User;
use crate::domain::user::user::CreateUserDTO;
use crate::traits::user::user_repository::UserRepository;
use crate::traits::user::pg_user_repository::PgUserRepository;

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
