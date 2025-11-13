use async_trait::async_trait;
use crate::domain::base::repositories::BaseQueries;
use crate::domain::users::entities::user::User;
use crate::domain::value_objects::{Email, UserId, Username};
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::CassandraSession;
use crate::infrastructure::persistence::cassandra::{
    execute_query_one_with_params, execute_query,
};
use serde_json::Value as JsonValue;
use scylla::frame::response::result::Row;
use scylla::transport::errors::QueryError;

#[async_trait]
pub trait UserQueries: BaseQueries<User, UserId> + Send + Sync {
    async fn find_by_email_cassandra(&self, email: &Email) -> Result<Option<User>, DomainError>;
    
    async fn find_by_username_cassandra(&self, username: &Username) -> Result<Option<User>, DomainError>;
}

pub struct CassandraUserQueries {
    session: CassandraSession,
}

impl CassandraUserQueries {
    pub fn new(session: CassandraSession) -> Self {
        Self { session }
    }
}

#[async_trait]
impl BaseQueries<User, UserId> for CassandraUserQueries {
    async fn find_by_id_cassandra(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let query = "SELECT user_id, username, email, created_at, updated_at FROM users WHERE user_id = ?";
        let uuid_val: uuid::Uuid = id.as_uuid();
        
        fn map_row_to_user(row: &Row) -> Result<User, QueryError> {
            let user_id: uuid::Uuid = row.columns[0]
                .as_ref()
                .and_then(|v| v.as_uuid())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid UUID".to_string()))?;
            let username: String = row.columns[1]
                .as_ref()
                .and_then(|v| v.as_text())
                .map(|s| s.to_string())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid username".to_string()))?;
            let email: String = row.columns[2]
                .as_ref()
                .and_then(|v| v.as_text())
                .map(|s| s.to_string())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid email".to_string()))?;
            let created_at: i64 = row.columns[3]
                .as_ref()
                .and_then(|v| v.as_bigint())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid timestamp".to_string()))?;
            let updated_at: i64 = row.columns[4]
                .as_ref()
                .and_then(|v| v.as_bigint())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid timestamp".to_string()))?;
            
            Ok(User {
                id: UserId::from_uuid(user_id),
                username: Username::new(username).map_err(|_| {
                    QueryError::InvalidMessage("Invalid username".to_string())
                })?,
                email: Email::new(email).map_err(|_| {
                    QueryError::InvalidMessage("Invalid email".to_string())
                })?,
                created_at: chrono::DateTime::from_timestamp(created_at / 1000, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::from_timestamp(updated_at / 1000, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
            })
        }
        
        let user = execute_query_one_with_params(&self.session, query, (uuid_val,), map_row_to_user)
            .await
            .map_err(|e| DomainError::ValidationError(format!("Cassandra error: {}", e)))?;
        
        Ok(user)
    }

    async fn find_all_cassandra(&self) -> Result<Vec<User>, DomainError> {
        let query = "SELECT user_id, username, email, created_at, updated_at FROM users";
        
        fn map_row_to_user(row: &Row) -> Result<User, QueryError> {
            let user_id: uuid::Uuid = row.columns[0]
                .as_ref()
                .and_then(|v| v.as_uuid())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid UUID".to_string()))?;
            let username: String = row.columns[1]
                .as_ref()
                .and_then(|v| v.as_text())
                .map(|s| s.to_string())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid username".to_string()))?;
            let email: String = row.columns[2]
                .as_ref()
                .and_then(|v| v.as_text())
                .map(|s| s.to_string())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid email".to_string()))?;
            let created_at: i64 = row.columns[3]
                .as_ref()
                .and_then(|v| v.as_bigint())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid timestamp".to_string()))?;
            let updated_at: i64 = row.columns[4]
                .as_ref()
                .and_then(|v| v.as_bigint())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid timestamp".to_string()))?;
            
            Ok(User {
                id: UserId::from_uuid(user_id),
                username: Username::new(username).map_err(|_| {
                    QueryError::InvalidMessage("Invalid username".to_string())
                })?,
                email: Email::new(email).map_err(|_| {
                    QueryError::InvalidMessage("Invalid email".to_string())
                })?,
                created_at: chrono::DateTime::from_timestamp(created_at / 1000, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::from_timestamp(updated_at / 1000, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
            })
        }
        
        let users = execute_query(&self.session, query, map_row_to_user)
            .await
            .map_err(|e| DomainError::ValidationError(format!("Cassandra error: {}", e)))?;
        
        Ok(users)
    }

    async fn find_by_filter_cassandra(&self, _filter: JsonValue) -> Result<Vec<User>, DomainError> {
        Err(DomainError::ValidationError("Filter queries not implemented yet".to_string()))
    }
}

#[async_trait]
impl UserQueries for CassandraUserQueries {
    async fn find_by_email_cassandra(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let query = "SELECT user_id, username, email, created_at, updated_at FROM users_by_email WHERE email = ?";
        let text_val: String = email.as_str().to_string();
        
        fn map_row_to_user(row: &Row) -> Result<User, QueryError> {
            let user_id: uuid::Uuid = row.columns[0]
                .as_ref()
                .and_then(|v| v.as_uuid())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid UUID".to_string()))?;
            let username: String = row.columns[1]
                .as_ref()
                .and_then(|v| v.as_text())
                .map(|s| s.to_string())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid username".to_string()))?;
            let email: String = row.columns[2]
                .as_ref()
                .and_then(|v| v.as_text())
                .map(|s| s.to_string())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid email".to_string()))?;
            let created_at: i64 = row.columns[3]
                .as_ref()
                .and_then(|v| v.as_bigint())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid timestamp".to_string()))?;
            let updated_at: i64 = row.columns[4]
                .as_ref()
                .and_then(|v| v.as_bigint())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid timestamp".to_string()))?;
            
            Ok(User {
                id: UserId::from_uuid(user_id),
                username: Username::new(username).map_err(|_| {
                    QueryError::InvalidMessage("Invalid username".to_string())
                })?,
                email: Email::new(email).map_err(|_| {
                    QueryError::InvalidMessage("Invalid email".to_string())
                })?,
                created_at: chrono::DateTime::from_timestamp(created_at / 1000, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::from_timestamp(updated_at / 1000, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
            })
        }
        
        let user = execute_query_one_with_params(&self.session, query, (text_val,), map_row_to_user)
            .await
            .map_err(|e| DomainError::ValidationError(format!("Cassandra error: {}", e)))?;
        
        Ok(user)
    }

    async fn find_by_username_cassandra(&self, username: &Username) -> Result<Option<User>, DomainError> {
        let query = "SELECT user_id, username, email, created_at, updated_at FROM users_by_username WHERE username = ?";
        let text_val: String = username.as_str().to_string();
        
        fn map_row_to_user(row: &Row) -> Result<User, QueryError> {
            let user_id: uuid::Uuid = row.columns[0]
                .as_ref()
                .and_then(|v| v.as_uuid())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid UUID".to_string()))?;
            let username: String = row.columns[1]
                .as_ref()
                .and_then(|v| v.as_text())
                .map(|s| s.to_string())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid username".to_string()))?;
            let email: String = row.columns[2]
                .as_ref()
                .and_then(|v| v.as_text())
                .map(|s| s.to_string())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid email".to_string()))?;
            let created_at: i64 = row.columns[3]
                .as_ref()
                .and_then(|v| v.as_bigint())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid timestamp".to_string()))?;
            let updated_at: i64 = row.columns[4]
                .as_ref()
                .and_then(|v| v.as_bigint())
                .ok_or_else(|| QueryError::InvalidMessage("Invalid timestamp".to_string()))?;
            
            Ok(User {
                id: UserId::from_uuid(user_id),
                username: Username::new(username).map_err(|_| {
                    QueryError::InvalidMessage("Invalid username".to_string())
                })?,
                email: Email::new(email).map_err(|_| {
                    QueryError::InvalidMessage("Invalid email".to_string())
                })?,
                created_at: chrono::DateTime::from_timestamp(created_at / 1000, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::from_timestamp(updated_at / 1000, 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
            })
        }
        
        let user = execute_query_one_with_params(&self.session, query, (text_val,), map_row_to_user)
            .await
            .map_err(|e| DomainError::ValidationError(format!("Cassandra error: {}", e)))?;
        
        Ok(user)
    }
}

