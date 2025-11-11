use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::Row;
use crate::infrastructure::config::PostgreSQLConfig;

pub type PostgreSQLPool = PgPool;

pub async fn create_pool(config: &PostgreSQLConfig) -> Result<PostgreSQLPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await
}

pub async fn execute_query<T, F>(
    pool: &PostgreSQLPool,
    query: &str,
    mapper: F,
) -> Result<Vec<T>, sqlx::Error>
where
    F: Fn(&sqlx::postgres::PgRow) -> Result<T, sqlx::Error>,
{
    let rows = sqlx::query(query).fetch_all(pool).await?;
    let mut results = Vec::new();
    for row in rows {
        results.push(mapper(&row)?);
    }
    Ok(results)
}

pub async fn execute_query_with_params<T, F>(
    pool: &PostgreSQLPool,
    query: &str,
    params: &[&(dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync)],
    mapper: F,
) -> Result<Vec<T>, sqlx::Error>
where
    F: Fn(&sqlx::postgres::PgRow) -> Result<T, sqlx::Error>,
{
    let mut query_builder = sqlx::query(query);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    let rows = query_builder.fetch_all(pool).await?;
    let mut results = Vec::new();
    for row in rows {
        results.push(mapper(&row)?);
    }
    Ok(results)
}

pub async fn execute_query_one<T, F>(
    pool: &PostgreSQLPool,
    query: &str,
    mapper: F,
) -> Result<Option<T>, sqlx::Error>
where
    F: Fn(&sqlx::postgres::PgRow) -> Result<T, sqlx::Error>,
{
    let row = sqlx::query(query).fetch_optional(pool).await?;
    match row {
        Some(r) => Ok(Some(mapper(&r)?)),
        None => Ok(None),
    }
}

pub async fn execute_statement(
    pool: &PostgreSQLPool,
    query: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(query).execute(pool).await?;
    Ok(result.rows_affected())
}

pub async fn execute_statement_with_params(
    pool: &PostgreSQLPool,
    query: &str,
    params: &[&(dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync)],
) -> Result<u64, sqlx::Error> {
    let mut query_builder = sqlx::query(query);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    let result = query_builder.execute(pool).await?;
    Ok(result.rows_affected())
}

pub async fn execute_batch_insert(
    pool: &PostgreSQLPool,
    query: &str,
    batch_params: Vec<Vec<&(dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync)>>,
) -> Result<u64, sqlx::Error> {
    let mut total_affected = 0u64;
    for params in batch_params {
        total_affected += execute_statement_with_params(pool, query, &params).await?;
    }
    Ok(total_affected)
}

pub async fn begin_transaction(
    pool: &PostgreSQLPool,
) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, sqlx::Error> {
    pool.begin().await
}

