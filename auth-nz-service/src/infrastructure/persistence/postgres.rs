use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use crate::infrastructure::config::DatabaseConfig;

pub type PostgresPool = PgPool;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PostgresPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await
}

