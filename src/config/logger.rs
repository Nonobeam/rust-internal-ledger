use deadpool_postgres::PoolError;
use crate::common::logger::app_error::AppError;

impl From<PoolError> for AppError {
    fn from(err: PoolError) -> Self {
        AppError::DbError(err.to_string())
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        let error_msg = if let Some(db_err) = err.as_db_error() {
            let detail = db_err.detail().unwrap_or("");
            format!("{} {}", db_err.message(), detail).trim().to_string()
        } else {
            err.to_string()
        };
        AppError::DbError(error_msg)
    }
}
