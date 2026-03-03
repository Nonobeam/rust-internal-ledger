use crate::common::logger::app_error::AppError;
use crate::domain::user::user::{User, CreateUserDTO};

pub trait UserRepository {
    async fn create(&self, user: &CreateUserDTO) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError>;
}