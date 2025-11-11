use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub app: AppConfig,
    pub features: FeaturesConfig,
    pub cache: CacheConfig,
    pub rate_limit: RateLimitConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub enable_caching: bool,
    pub enable_rate_limiting: bool,
    pub enable_metrics: bool,
    pub enable_tracing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub ttl_seconds: u64,
    pub max_size: usize,
    pub enable_redis: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub enable_file_logging: bool,
    pub log_file_path: Option<String>,
}

impl ApplicationConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "src/application/configs/configs.yaml".to_string());

        Self::load_from_file(&config_path)
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: ApplicationConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: ApplicationConfig = serde_yaml::from_str(content)?;
        Ok(config)
    }

    pub fn default() -> Self {
        Self {
            app: AppConfig {
                name: "user-service".to_string(),
                version: "0.1.0".to_string(),
                environment: std::env::var("ENVIRONMENT")
                    .unwrap_or_else(|_| "development".to_string()),
                debug: std::env::var("DEBUG")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
            },
            features: FeaturesConfig {
                enable_caching: true,
                enable_rate_limiting: true,
                enable_metrics: true,
                enable_tracing: true,
            },
            cache: CacheConfig {
                ttl_seconds: 3600,
                max_size: 10000,
                enable_redis: true,
            },
            rate_limit: RateLimitConfig {
                enabled: true,
                requests_per_minute: 100,
                burst_size: 20,
            },
            logging: LoggingConfig {
                level: std::env::var("LOG_LEVEL")
                    .unwrap_or_else(|_| "info".to_string()),
                format: "json".to_string(),
                enable_file_logging: false,
                log_file_path: None,
            },
        }
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self::default()
    }
}

