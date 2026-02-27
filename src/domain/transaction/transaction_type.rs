use chrono::{DateTime, Utc};

pub struct TransactionType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TransactionType {
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            created_at: Utc::now(),
        }
    }
}
