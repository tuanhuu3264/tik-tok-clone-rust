use redis::{Client, RedisResult};
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Duration;
use crate::infrastructure::config::RedisConfig;

pub type RedisConnection = redis::aio::Connection;

pub struct RedisCache {
    client: Client,
    default_ttl: Option<Duration>,
}

impl RedisCache {
    pub fn new(config: &RedisConfig) -> RedisResult<Self> {
        let default_ttl = config.default_ttl_seconds.map(Duration::from_secs);
        
        if config.cluster_mode {
            return Err(redis::RedisError::from((
                redis::ErrorKind::InvalidClientConfig,
                "Cluster mode is not yet supported in this implementation",
            )));
        }
        
        let client = Client::open(config.url.as_str())?;
        
        Ok(Self {
            client,
            default_ttl,
        })
    }

    pub async fn get_connection(&self) -> RedisResult<RedisConnection> {
        let conn = self.client.get_async_connection().await?;
        Ok(conn)
    }

    pub async fn get<T>(&self, key: &str) -> RedisResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut conn = self.get_connection().await?;
        let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        
        match value {
            Some(v) => {
                let deserialized: T = serde_json::from_str(&v)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "JSON deserialization error")))?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T) -> RedisResult<()>
    where
        T: Serialize,
    {
        self.set_with_ttl(key, value, self.default_ttl).await
    }

    pub async fn set_with_ttl<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> RedisResult<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_string(value)
            .map_err(|_| redis::RedisError::from((redis::ErrorKind::TypeError, "JSON serialization error")))?;
        
        let mut conn = self.get_connection().await?;
        match ttl {
            Some(duration) => {
                redis::cmd("SETEX")
                    .arg(key)
                    .arg(duration.as_secs())
                    .arg(&serialized)
                    .query_async(&mut conn)
                    .await?;
            }
            None => {
                redis::cmd("SET")
                    .arg(key)
                    .arg(&serialized)
                    .query_async(&mut conn)
                    .await?;
            }
        }
        
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.get_connection().await?;
        let deleted: u32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        Ok(deleted > 0)
    }

    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.get_connection().await?;
        let exists: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
        Ok(exists)
    }

    pub async fn expire(&self, key: &str, seconds: u64) -> RedisResult<bool> {
        let mut conn = self.get_connection().await?;
        let result: bool = redis::cmd("EXPIRE").arg(key).arg(seconds).query_async(&mut conn).await?;
        Ok(result)
    }

    pub async fn ttl(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.get_connection().await?;
        let ttl: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
        Ok(ttl)
    }

    pub async fn set_string(&self, key: &str, value: &str) -> RedisResult<()> {
        self.set_string_with_ttl(key, value, self.default_ttl).await
    }

    pub async fn set_string_with_ttl(&self, key: &str, value: &str, ttl: Option<Duration>) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        match ttl {
            Some(duration) => {
                redis::cmd("SETEX")
                    .arg(key)
                    .arg(duration.as_secs())
                    .arg(value)
                    .query_async(&mut conn)
                    .await?;
            }
            None => {
                redis::cmd("SET")
                    .arg(key)
                    .arg(value)
                    .query_async(&mut conn)
                    .await?;
            }
        }
        
        Ok(())
    }

    pub async fn get_string(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.get_connection().await?;
        let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        Ok(value)
    }

    pub async fn increment(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.get_connection().await?;
        let value: i64 = redis::cmd("INCR").arg(key).query_async(&mut conn).await?;
        Ok(value)
    }

    pub async fn increment_by(&self, key: &str, amount: i64) -> RedisResult<i64> {
        let mut conn = self.get_connection().await?;
        let value: i64 = redis::cmd("INCRBY").arg(key).arg(amount).query_async(&mut conn).await?;
        Ok(value)
    }

    pub async fn decrement(&self, key: &str) -> RedisResult<i64> {
        let mut conn = self.get_connection().await?;
        let value: i64 = redis::cmd("DECR").arg(key).query_async(&mut conn).await?;
        Ok(value)
    }

    pub async fn keys(&self, pattern: &str) -> RedisResult<Vec<String>> {
        let mut conn = self.get_connection().await?;
        let keys: Vec<String> = redis::cmd("KEYS").arg(pattern).query_async(&mut conn).await?;
        Ok(keys)
    }

    pub async fn flush_db(&self) -> RedisResult<()> {
        let mut conn = self.get_connection().await?;
        redis::cmd("FLUSHDB").query_async(&mut conn).await?;
        Ok(())
    }

    pub async fn ping(&self) -> RedisResult<String> {
        let mut conn = self.get_connection().await?;
        let result: String = redis::cmd("PING").query_async(&mut conn).await?;
        Ok(result)
    }

    pub fn is_cluster(&self) -> bool {
        false
    }
}

pub async fn create_redis_cache(config: &RedisConfig) -> RedisResult<RedisCache> {
    RedisCache::new(config)
}
