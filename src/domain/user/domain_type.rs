use chrono::{DateTime, Utc};

pub struct DomainType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

impl DomainType {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            created_at: Utc::now(),
        }
    }
}