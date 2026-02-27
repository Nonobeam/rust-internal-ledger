use deadpool_postgres::{Config as PgConfig, Pool, Runtime};
use std::env;
use tokio_postgres::NoTls;

pub struct AppConfig {
    pub pg: PgConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let mut pg = PgConfig::new();
        
        pg.host = Some(
            env::var("PG__HOST")
                .expect("PG__HOST must be set")
        );
        pg.user = Some(
            env::var("PG__USER")
                .expect("PG__USER must be set")
        );
        pg.password = Some(
            env::var("PG__PASSWORD")
                .expect("PG__PASSWORD must be set")
        );
        pg.dbname = Some(
            env::var("PG__DBNAME")
                .expect("PG__DBNAME must be set")
        );
        pg.port = Some(
            env::var("PG__PORT")
                .expect("PG__PORT must be set")
                .parse::<u16>()
                .expect("PG__PORT must be a valid number")
        );
        pg.options = Some(
            format!("-c search_path={}", env::var("PG__SCHEMA").unwrap_or("central_ledger".to_string()))
        );

        AppConfig { pg }
    }
}

pub fn create_pool(config: &AppConfig) -> Pool {
    config.pg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create pool")
}