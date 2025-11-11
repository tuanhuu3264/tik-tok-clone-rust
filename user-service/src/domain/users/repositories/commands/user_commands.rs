use async_trait::async_trait;
use crate::domain::base::repositories::BaseCommands;
use crate::domain::users::entities::user::User;
use crate::domain::value_objects::UserId;
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::PostgreSQLPool;

#[async_trait]
pub trait UserCommands: BaseCommands<User, UserId> + Send + Sync {
    async fn create_user(&self, user: &User) -> Result<(), DomainError>;
    
    async fn update_user(&self, user: &User) -> Result<(), DomainError>;
    
    async fn delete_user(&self, id: &UserId) -> Result<(), DomainError>;
}

pub struct PostgreSQLUserCommands {
    pool: PostgreSQLPool,
}

impl PostgreSQLUserCommands {
    pub fn new(pool: PostgreSQLPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BaseCommands<User, UserId> for PostgreSQLUserCommands {
    async fn insert_postgres(&self, entity: &User) -> Result<(), DomainError> {
        self.create_user(entity).await
    }

    async fn update_postgres(&self, id: &UserId, entity: &User) -> Result<(), DomainError> {
        self.update_user(entity).await
    }

    async fn delete_postgres(&self, id: &UserId) -> Result<(), DomainError> {
        self.delete_user(id).await
    }

    async fn batch_insert_postgres(&self, entities: &[User]) -> Result<(), DomainError> {
        for entity in entities {
            self.create_user(entity).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl UserCommands for PostgreSQLUserCommands {
    async fn create_user(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id.as_uuid(),
            user.username.as_str(),
            user.email.as_str(),
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(format!("PostgreSQL error: {}", e)))?;
        
        Ok(())
    }

    async fn update_user(&self, user: &User) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE users
            SET username = $2, email = $3, updated_at = $4
            WHERE id = $1
            "#,
            user.id.as_uuid(),
            user.username.as_str(),
            user.email.as_str(),
            user.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(format!("PostgreSQL error: {}", e)))?;
        
        Ok(())
    }

    async fn delete_user(&self, id: &UserId) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(format!("PostgreSQL error: {}", e)))?;
        
        Ok(())
    }
}

