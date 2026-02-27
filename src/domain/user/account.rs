use chrono::{DateTime, Utc};

pub struct Account {
    pub id: String,
    pub owner_id: String,

    pub domain: String,
    pub currency: String,
    pub bucket_type: String,

    pub created_at: DateTime<Utc>,
}

impl Account {
    pub fn new(id: String, owner_id: String, domain: String, currency: String, bucket_type: String) -> Self {
        Self {
            id,
            owner_id,
            domain,
            currency,
            bucket_type,
            created_at: Utc::now(),
        }
    }
}