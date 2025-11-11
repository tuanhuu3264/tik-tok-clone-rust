use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::profiles::entities::profile::Profile;
use crate::domain::profiles::repositories::ProfileRepository;
use crate::domain::value_objects::UserId;
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::PostgreSQLPool;

pub struct PostgresProfileRepository {
    pool: PostgreSQLPool,
}

impl PostgresProfileRepository {
    pub fn new(pool: PostgreSQLPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfileRepository for PostgresProfileRepository {
    async fn create(&self, profile: &Profile) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO profiles (user_id, display_name, bio, avatar_url, created_at, updated_at, status_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            profile.user_id.as_uuid(),
            profile.display_name.as_deref(),
            profile.bio.as_deref(),
            profile.avatar_url.as_deref(),
            profile.created_at,
            profile.updated_at,
            profile.status_id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(format!("PostgreSQL error: {}", e)))?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<Profile>, DomainError> {
        let row = sqlx::query!(
            r#"
            SELECT user_id, display_name, bio, avatar_url, created_at, updated_at, status_id
            FROM profiles
            WHERE user_id = $1
            "#,
            user_id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(format!("PostgreSQL error: {}", e)))?;

        Ok(row.map(|r| Profile {
            user_id: UserId::from_uuid(r.user_id),
            display_name: r.display_name,
            bio: r.bio,
            avatar_url: r.avatar_url,
            created_at: r.created_at,
            updated_at: r.updated_at,
            status_id: crate::domain::value_objects::StatusId::from_uuid(r.status_id),
        }))
    }

    async fn update(&self, profile: &Profile) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE profiles
            SET display_name = $2, bio = $3, avatar_url = $4, updated_at = $5, status_id = $6
            WHERE user_id = $1
            "#,
            profile.user_id.as_uuid(),
            profile.display_name.as_deref(),
            profile.bio.as_deref(),
            profile.avatar_url.as_deref(),
            profile.updated_at,
            profile.status_id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(format!("PostgreSQL error: {}", e)))?;

        Ok(())
    }

    async fn delete(&self, user_id: &UserId) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM profiles WHERE user_id = $1",
            user_id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(format!("PostgreSQL error: {}", e)))?;

        Ok(())
    }
}

