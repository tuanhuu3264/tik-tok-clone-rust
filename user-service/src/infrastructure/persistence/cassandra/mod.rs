use std::sync::Arc;
use scylla::{Session, SessionBuilder};
use scylla::transport::errors::QueryError;
use crate::infrastructure::config::CassandraConfig;

pub type CassandraSession = Arc<Session>;

pub async fn create_session(config: &CassandraConfig) -> Result<CassandraSession, QueryError> {
    let mut session_builder = SessionBuilder::new()
        .known_node(&config.url);

    if let (Some(username), Some(password)) = (&config.username, &config.password) {
        session_builder = session_builder.user(username, password);
    }

    let session = session_builder.build().await?;

    if let Some(keyspace) = &config.keyspace {
        session.use_keyspace(keyspace, false).await?;
    }

    Ok(Arc::new(session))
}

pub async fn execute_query<T, F>(
    session: &CassandraSession,
    query: &str,
    mapper: F,
) -> Result<Vec<T>, QueryError>
where
    F: Fn(&scylla::frame::response::result::Row) -> Result<T, QueryError>,
{
    let result = session.query(query, &[]).await?;
    let mut results = Vec::new();
    for row in result.rows().unwrap_or(&[]) {
        results.push(mapper(row)?);
    }
    Ok(results)
}

pub async fn execute_query_with_params<T, F>(
    session: &CassandraSession,
    query: &str,
    params: &[scylla::frame::value::Value],
    mapper: F,
) -> Result<Vec<T>, QueryError>
where
    F: Fn(&scylla::frame::response::result::Row) -> Result<T, QueryError>,
{
    let result = session.query(query, params).await?;
    let mut results = Vec::new();
    for row in result.rows().unwrap_or(&[]) {
        results.push(mapper(row)?);
    }
    Ok(results)
}

pub async fn execute_query_one<T, F>(
    session: &CassandraSession,
    query: &str,
    mapper: F,
) -> Result<Option<T>, QueryError>
where
    F: Fn(&scylla::frame::response::result::Row) -> Result<T, QueryError>,
{
    let results = execute_query(session, query, mapper).await?;
    Ok(results.into_iter().next())
}

pub async fn execute_query_one_with_params<T, F>(
    session: &CassandraSession,
    query: &str,
    params: &[scylla::frame::value::Value],
    mapper: F,
) -> Result<Option<T>, QueryError>
where
    F: Fn(&scylla::frame::response::result::Row) -> Result<T, QueryError>,
{
    let results = execute_query_with_params(session, query, params, mapper).await?;
    Ok(results.into_iter().next())
}

pub async fn execute_statement(
    session: &CassandraSession,
    query: &str,
) -> Result<(), QueryError> {
    session.query(query, &[]).await?;
    Ok(())
}

pub async fn execute_statement_with_params(
    session: &CassandraSession,
    query: &str,
    params: &[scylla::frame::value::Value],
) -> Result<(), QueryError> {
    session.query(query, params).await?;
    Ok(())
}

