use crate::domain::errors::DomainError;
use std::sync::Arc;
use crate::infrastructure::persistence::RedisCache;

pub struct LogoutUseCase {
    redis_cache: Arc<RedisCache>,
}

impl LogoutUseCase {
    pub fn new(redis_cache: Arc<RedisCache>) -> Self {
        Self { redis_cache }
    }

    pub async fn execute(&self, access_token: String) -> Result<(), DomainError> {
        let token_key = format!("token:{}", access_token);
        
        self.redis_cache
            .set_string_with_ttl(&token_key, "revoked", Some(std::time::Duration::from_secs(3600)))
            .await
            .map_err(|e| DomainError::ValidationError(format!("Redis error: {}", e)))?;

        Ok(())
    }
}

