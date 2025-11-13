use std::sync::Arc;
use crate::infrastructure::config::Config;
use crate::infrastructure::persistence::{PostgreSQLPool, CassandraSession, RedisCache};
use crate::infrastructure::persistence::postgreSQL::create_pool as create_postgresql_pool;
use crate::infrastructure::persistence::cassandra::create_session as create_cassandra_session;
use crate::infrastructure::persistence::redis_cache::create_redis_cache;
use crate::infrastructure::repositories::{UserRepositoryImpl, PostgresProfileRepository};
use crate::domain::users::repositories::UserRepository;
use crate::domain::profiles::repositories::ProfileRepository;

#[derive(Clone)]
pub struct AppContext {
    pub user_repository: Arc<dyn UserRepository>,
    pub profile_repository: Arc<dyn ProfileRepository>,
    pub redis_cache: Arc<RedisCache>,
}

impl AppContext {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let postgres_pool = create_postgresql_pool(
            config.postgresql.as_ref().ok_or("PostgreSQL config not found")?
        ).await?;

        let cassandra_session = create_cassandra_session(
            config.cassandra.as_ref().ok_or("Cassandra config not found")?
        ).await?;

        let redis_config = config.redis.as_ref().ok_or("Redis config not found")?;
        let redis_cache = Arc::new(create_redis_cache(redis_config).await?);

        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserRepositoryImpl::new(postgres_pool.clone(), cassandra_session.clone())
        );

        let profile_repository: Arc<dyn ProfileRepository> = Arc::new(
            PostgresProfileRepository::new(postgres_pool)
        );

        Ok(Self {
            user_repository,
            profile_repository,
            redis_cache,
        })
    }
}

