use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::users::entities::user::User;
use crate::domain::users::repositories::{UserRepository, UserCommands, UserQueries};
use crate::domain::base::repositories::BaseQueries;
use crate::domain::users::repositories::commands::PostgreSQLUserCommands;
use crate::domain::users::repositories::queries::CassandraUserQueries;
use crate::domain::value_objects::{Email, UserId, Username};
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::{PostgreSQLPool, CassandraSession};

pub struct UserRepositoryImpl {
    commands: Arc<PostgreSQLUserCommands>,
    queries: Arc<CassandraUserQueries>,
}

impl UserRepositoryImpl {
    pub fn new(postgres_pool: PostgreSQLPool, cassandra_session: CassandraSession) -> Self {
        Self {
            commands: Arc::new(PostgreSQLUserCommands::new(postgres_pool)),
            queries: Arc::new(CassandraUserQueries::new(cassandra_session)),
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        self.commands.create_user(user).await
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        self.queries.find_by_id_cassandra(id).await
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        self.queries.find_by_email_cassandra(email).await
    }

    async fn find_by_username(&self, username: &Username) -> Result<Option<User>, DomainError> {
        self.queries.find_by_username_cassandra(username).await
    }

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        self.commands.update_user(user).await
    }

    async fn delete(&self, id: &UserId) -> Result<(), DomainError> {
        self.commands.delete_user(id).await
    }
}
