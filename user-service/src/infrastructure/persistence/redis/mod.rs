use redis::aio::{ConnectionManager, ClusterConnection};
use redis::{Client, ClusterClient, RedisResult};
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Duration;
use crate::infrastructure::config::RedisConfig;

pub enum RedisConnection {
    Single(ConnectionManager),
    Cluster(ClusterConnection),
}

pub struct RedisCache {
    client: Option<Client>,
    cluster_client: Option<ClusterClient>,
    default_ttl: Option<Duration>,
    is_cluster: bool,
}

impl RedisCache {
    pub fn new(config: &RedisConfig) -> RedisResult<Self> {
        let default_ttl = config.default_ttl_seconds.map(Duration::from_secs);
        
        if config.cluster_mode {
            let nodes = config.cluster_nodes.as_ref()
                .map(|nodes| nodes.clone())
                .unwrap_or_else(|| vec![config.url.clone()]);
            
            let cluster_client = ClusterClient::new(nodes)?;
            
            Ok(Self {
                client: None,
                cluster_client: Some(cluster_client),
                default_ttl,
                is_cluster: true,
            })
        } else {
            let client = Client::open(config.url.as_str())?;
            
            Ok(Self {
                client: Some(client),
                cluster_client: None,
                default_ttl,
                is_cluster: false,
            })
        }
    }

    pub async fn get_connection(&self) -> RedisResult<RedisConnection> {
        if self.is_cluster {
            let cluster_client = self.cluster_client.as_ref().unwrap();
            let conn = cluster_client.get_async_connection().await?;
            Ok(RedisConnection::Cluster(conn))
        } else {
            let client = self.client.as_ref().unwrap();
            let conn = client.get_connection_manager().await?;
            Ok(RedisConnection::Single(conn))
        }
    }

    pub async fn get<T>(&self, key: &str) -> RedisResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let value: Option<String> = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("GET").arg(key).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("GET").arg(key).query_async(conn).await?
            }
        };
        
        match value {
            Some(v) => {
                let deserialized: T = serde_json::from_str(&v)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, e.to_string())))?;
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
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, e.to_string())))?;
        
        match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                match ttl {
                    Some(duration) => {
                        redis::cmd("SETEX")
                            .arg(key)
                            .arg(duration.as_secs())
                            .arg(&serialized)
                            .query_async(conn)
                            .await?;
                    }
                    None => {
                        redis::cmd("SET")
                            .arg(key)
                            .arg(&serialized)
                            .query_async(conn)
                            .await?;
                    }
                }
            }
            RedisConnection::Cluster(conn) => {
                match ttl {
                    Some(duration) => {
                        redis::cmd("SETEX")
                            .arg(key)
                            .arg(duration.as_secs())
                            .arg(&serialized)
                            .query_async(conn)
                            .await?;
                    }
                    None => {
                        redis::cmd("SET")
                            .arg(key)
                            .arg(&serialized)
                            .query_async(conn)
                            .await?;
                    }
                }
            }
        }
        
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> RedisResult<bool> {
        let deleted: u32 = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("DEL").arg(key).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("DEL").arg(key).query_async(conn).await?
            }
        };
        Ok(deleted > 0)
    }

    pub async fn exists(&self, key: &str) -> RedisResult<bool> {
        let exists: bool = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("EXISTS").arg(key).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("EXISTS").arg(key).query_async(conn).await?
            }
        };
        Ok(exists)
    }

    pub async fn expire(&self, key: &str, seconds: u64) -> RedisResult<bool> {
        let result: bool = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("EXPIRE").arg(key).arg(seconds).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("EXPIRE").arg(key).arg(seconds).query_async(conn).await?
            }
        };
        Ok(result)
    }

    pub async fn ttl(&self, key: &str) -> RedisResult<i64> {
        let ttl: i64 = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("TTL").arg(key).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("TTL").arg(key).query_async(conn).await?
            }
        };
        Ok(ttl)
    }

    pub async fn set_string(&self, key: &str, value: &str) -> RedisResult<()> {
        self.set_string_with_ttl(key, value, self.default_ttl).await
    }

    pub async fn set_string_with_ttl(&self, key: &str, value: &str, ttl: Option<Duration>) -> RedisResult<()> {
        match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                match ttl {
                    Some(duration) => {
                        redis::cmd("SETEX")
                            .arg(key)
                            .arg(duration.as_secs())
                            .arg(value)
                            .query_async(conn)
                            .await?;
                    }
                    None => {
                        redis::cmd("SET")
                            .arg(key)
                            .arg(value)
                            .query_async(conn)
                            .await?;
                    }
                }
            }
            RedisConnection::Cluster(conn) => {
                match ttl {
                    Some(duration) => {
                        redis::cmd("SETEX")
                            .arg(key)
                            .arg(duration.as_secs())
                            .arg(value)
                            .query_async(conn)
                            .await?;
                    }
                    None => {
                        redis::cmd("SET")
                            .arg(key)
                            .arg(value)
                            .query_async(conn)
                            .await?;
                    }
                }
            }
        }
        
        Ok(())
    }

    pub async fn get_string(&self, key: &str) -> RedisResult<Option<String>> {
        let value: Option<String> = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("GET").arg(key).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("GET").arg(key).query_async(conn).await?
            }
        };
        Ok(value)
    }

    pub async fn increment(&self, key: &str) -> RedisResult<i64> {
        let value: i64 = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("INCR").arg(key).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("INCR").arg(key).query_async(conn).await?
            }
        };
        Ok(value)
    }

    pub async fn increment_by(&self, key: &str, amount: i64) -> RedisResult<i64> {
        let value: i64 = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("INCRBY").arg(key).arg(amount).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("INCRBY").arg(key).arg(amount).query_async(conn).await?
            }
        };
        Ok(value)
    }

    pub async fn decrement(&self, key: &str) -> RedisResult<i64> {
        let value: i64 = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("DECR").arg(key).query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("DECR").arg(key).query_async(conn).await?
            }
        };
        Ok(value)
    }

    pub async fn keys(&self, pattern: &str) -> RedisResult<Vec<String>> {
        if self.is_cluster {
            return Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "KEYS command is not supported in cluster mode. Use SCAN instead.".to_string(),
            )));
        }
        
        let keys: Vec<String> = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("KEYS").arg(pattern).query_async(conn).await?
            }
            RedisConnection::Cluster(_) => {
                return Err(redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "KEYS command is not supported in cluster mode".to_string(),
                )));
            }
        };
        Ok(keys)
    }

    pub async fn flush_db(&self) -> RedisResult<()> {
        if self.is_cluster {
            return Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "FLUSHDB command is not supported in cluster mode".to_string(),
            )));
        }
        
        match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("FLUSHDB").query_async(conn).await?;
            }
            RedisConnection::Cluster(_) => {
                return Err(redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "FLUSHDB command is not supported in cluster mode".to_string(),
                )));
            }
        }
        Ok(())
    }

    pub async fn ping(&self) -> RedisResult<String> {
        let result: String = match &mut self.get_connection().await? {
            RedisConnection::Single(conn) => {
                redis::cmd("PING").query_async(conn).await?
            }
            RedisConnection::Cluster(conn) => {
                redis::cmd("PING").query_async(conn).await?
            }
        };
        Ok(result)
    }

    pub fn is_cluster(&self) -> bool {
        self.is_cluster
    }
}

pub async fn create_redis_cache(config: &RedisConfig) -> RedisResult<RedisCache> {
    RedisCache::new(config)
}
