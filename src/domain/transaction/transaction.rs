use chrono::{DateTime, Utc};

pub struct Transaction {
    pub id: String,
    pub idempotency_key: String,
    pub transaction_type: String,
    pub reference_id: Option<String>,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        id: String,
        idempotency_key: String,
        transaction_type: String,
        reference_id: Option<String>,
        metadata: Option<String>,
    ) -> Self {
        Self {
            id,
            idempotency_key,
            transaction_type,
            reference_id,
            metadata,
            created_at: Utc::now(),
        }
    }
}
