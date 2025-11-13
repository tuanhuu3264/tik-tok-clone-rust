use crate::domain::users::entities::user::User;
use crate::domain::users::repositories::UserRepository;
use crate::domain::profiles::repositories::ProfileRepository;
use crate::domain::value_objects::{Email, Username, UserId, StatusId};
use crate::domain::errors::DomainError;
use crate::domain::credentials::entities::credentials::Credentials;
use crate::domain::passwords::entities::Password;
use chrono::{Duration, Utc};
use argon2::{Argon2, PasswordHasher as _};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub struct RegisterUseCase {
    user_repository: Arc<dyn UserRepository>,
    profile_repository: Arc<dyn ProfileRepository>,
}

impl RegisterUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        profile_repository: Arc<dyn ProfileRepository>,
    ) -> Self {
        Self {
            user_repository,
            profile_repository,
        }
    }

    pub async fn execute(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<RegisterResult, DomainError> {
        let email_vo = Email::new(email)?;
        let username_vo = Username::new(username)?;

        if self.user_repository.find_by_email(&email_vo).await?.is_some() {
            return Err(DomainError::UserAlreadyExists);
        }

        if self.user_repository.find_by_username(&username_vo).await?.is_some() {
            return Err(DomainError::UserAlreadyExists);
        }

        let user_id = UserId::new();
        let user = User::new(user_id, username_vo, email_vo);
        
        self.user_repository.create(&user).await?;

        let password_hash = Self::hash_password(&password)?;
        let _password = Password::new(user_id, password_hash);

        let status_id = StatusId::new();
        let profile = crate::domain::profiles::entities::profile::Profile::new(user_id, status_id);
        self.profile_repository.create(&profile).await?;

        let tokens = Self::generate_tokens(&user_id)?;
        let credentials = Credentials::new_with_tokens(
            user_id,
            tokens.access_token.clone(),
            tokens.refresh_token.clone(),
            tokens.access_token_expires_at,
            tokens.refresh_token_expires_at,
            None,
        );

        Ok(RegisterResult {
            user,
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            access_token_expires_at: tokens.access_token_expires_at,
            refresh_token_expires_at: tokens.refresh_token_expires_at,
        })
    }

    fn hash_password(password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| DomainError::ValidationError(format!("Password hashing error: {}", e)))?;
        Ok(password_hash.to_string())
    }

    fn generate_tokens(user_id: &UserId) -> Result<TokenResult, DomainError> {
        let now = Utc::now();
        let access_token_expires_at = now + Duration::hours(1);
        let refresh_token_expires_at = now + Duration::days(7);

        let access_claims = Claims {
            sub: user_id.as_uuid().to_string(),
            exp: access_token_expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let refresh_claims = Claims {
            sub: user_id.as_uuid().to_string(),
            exp: refresh_token_expires_at.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

        let access_token = encode(
            &Header::new(Algorithm::HS256),
            &access_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| DomainError::ValidationError(format!("Token encoding error: {}", e)))?;

        let refresh_token = encode(
            &Header::new(Algorithm::HS256),
            &refresh_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| DomainError::ValidationError(format!("Token encoding error: {}", e)))?;

        Ok(TokenResult {
            access_token,
            refresh_token,
            access_token_expires_at,
            refresh_token_expires_at,
        })
    }
}

pub struct RegisterResult {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_at: chrono::DateTime<Utc>,
    pub refresh_token_expires_at: chrono::DateTime<Utc>,
}

struct TokenResult {
    access_token: String,
    refresh_token: String,
    access_token_expires_at: chrono::DateTime<Utc>,
    refresh_token_expires_at: chrono::DateTime<Utc>,
}

