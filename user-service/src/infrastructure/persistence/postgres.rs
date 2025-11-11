use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::infrastructure::config::{DatabaseConfig, PostgreSQLConfig};

pub type PostgresPool = PgPool;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PostgresPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await
}

pub async fn create_pool_from_config(config: &PostgreSQLConfig) -> Result<PostgresPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await
}

