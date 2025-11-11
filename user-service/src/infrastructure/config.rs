use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub postgresql: Option<PostgreSQLConfig>,
    pub cassandra: Option<CassandraConfig>,
    pub redis: Option<RedisConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgreSQLConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CassandraConfig {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub keyspace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub cluster_mode: bool,
    pub cluster_nodes: Option<Vec<String>>,
    pub max_connections: Option<u32>,
    pub default_ttl_seconds: Option<u64>,
    pub connection_timeout_seconds: Option<u64>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // In production, load from environment variables or config file
        Ok(Self {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .unwrap_or(3000),
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgresql://user:password@localhost/userdb".to_string()),
                max_connections: 10,
                username: std::env::var("DB_USERNAME").ok(),
                password: std::env::var("DB_PASSWORD").ok(),
            },
            postgresql: Some(PostgreSQLConfig {
                url: std::env::var("POSTGRESQL_URL")
                    .unwrap_or_else(|_| "postgresql://user:password@localhost/userdb".to_string()),
                max_connections: 10,
            }),
            cassandra: Some(CassandraConfig {
                url: std::env::var("CASSANDRA_URL")
                    .unwrap_or_else(|_| "127.0.0.1:9042".to_string()),
                username: std::env::var("CASSANDRA_USERNAME").ok(),
                password: std::env::var("CASSANDRA_PASSWORD").ok(),
                keyspace: std::env::var("CASSANDRA_KEYSPACE").ok(),
            }),
            redis: Some(RedisConfig {
                url: std::env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
                cluster_mode: std::env::var("REDIS_CLUSTER_MODE")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                cluster_nodes: std::env::var("REDIS_CLUSTER_NODES")
                    .ok()
                    .map(|s| s.split(',').map(|n| n.trim().to_string()).collect()),
                max_connections: std::env::var("REDIS_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|s| s.parse().ok()),
                default_ttl_seconds: std::env::var("REDIS_DEFAULT_TTL")
                    .ok()
                    .and_then(|s| s.parse().ok()),
                connection_timeout_seconds: std::env::var("REDIS_TIMEOUT")
                    .ok()
                    .and_then(|s| s.parse().ok()),
            }),
        })
    }
}

