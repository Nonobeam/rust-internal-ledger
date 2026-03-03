use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::traits::user::user_repository::UserRepository;
use crate::domain::user::user::{User, CreateUserDTO};
use crate::common::logger::app_error::AppError;

pub struct PgUserRepository {
    pub db: Arc<Pool>
}

impl UserRepository for PgUserRepository {
    async fn create(&self, user: &CreateUserDTO) -> Result<Option<User>, AppError> {
        let client = self.db.get().await?;
        let row = client
            .query_one(
                "SELECT id FROM user_types WHERE name = $1",
                &[&user.user_type],
            )
            .await?;

        let user_type_id: Uuid = row.get("id");
        let row = client
            .query_opt("INSERT INTO users (name, user_type) VALUES ($1, $2) RETURNING id, name, user_type, created_at", &[&user.name, &user_type_id])
            .await?;
        Ok(row.map(|r| User {
            id: r.get("id"),
            name: r.get("name"),
            user_type: r.get("user_type"),
            created_at: r.get("created_at"),
        }))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let client = self.db.get().await?;
        let row = client
            .query_opt("SELECT id, name, user_type, created_at FROM users WHERE id = $1", &[&id])
            .await?;
        Ok(row.map(|r| User {
            id: r.get("id"),
            name: r.get("name"),
            user_type: r.get("user_type"),
            created_at: r.get("created_at"),
        }))
    }
}