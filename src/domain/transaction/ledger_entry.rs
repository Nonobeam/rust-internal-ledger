use chrono::{DateTime, Utc};

pub struct LedgerEntry {
    pub id: String,
    pub seq: i64,

    pub transaction_id: String,
    pub account_id: String,

    pub credit: Option<f64>,
    pub debit: Option<f64>,

    pub created_at: DateTime<Utc>,
}

impl LedgerEntry {
    pub fn new(
        id: String,
        seq: i64,
        transaction_id: String,
        account_id: String,
        credit: Option<f64>,
        debit: Option<f64>,
    ) -> Self {
        Self {
            id,
            seq,
            transaction_id,
            account_id,
            credit,
            debit,
            created_at: Utc::now(),
        }
    }
}
