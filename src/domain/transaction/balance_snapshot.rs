use chrono::{DateTime, Utc};

pub struct BalanceSnapshot {
    pub owner_id: String,
    pub domain: String,
    pub currency: String,

    pub available_balance: f64,
    pub reserved_balance: f64,

    pub last_available_entry_id: Option<String>,
    pub last_reserved_entry_id: Option<String>,

    pub updated_at: DateTime<Utc>,
}

impl BalanceSnapshot {
    pub fn new(
        owner_id: String,
        domain: String,
        currency: String,
        available_balance: f64,
        reserved_balance: f64,
        last_available_entry_id: Option<String>,
        last_reserved_entry_id: Option<String>,
    ) -> Self {
        Self {
            owner_id,
            domain,
            currency,
            available_balance,
            reserved_balance,
            last_available_entry_id,
            last_reserved_entry_id,
            updated_at: Utc::now(),
        }
    }
}
