use crate::domain::users::repositories::UserRepository;
use crate::domain::value_objects::Email;
use crate::domain::errors::DomainError;
use chrono::{Duration, Utc};
use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub struct LoginUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl LoginUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        email: String,
        password: String,
    ) -> Result<LoginResult, DomainError> {
        let email_vo = Email::new(email)?;
        let user = self.user_repository
            .find_by_email(&email_vo)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        let password_hash = LoginUseCase::get_password_hash(&user.id).await?;
        if !Self::verify_password(&password, &password_hash)? {
            return Err(DomainError::ValidationError("Invalid password".to_string()));
        }

        let tokens = Self::generate_tokens(&user.id)?;

        Ok(LoginResult {
            user,
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            access_token_expires_at: tokens.access_token_expires_at,
            refresh_token_expires_at: tokens.refresh_token_expires_at,
        })
    }

    async fn get_password_hash(_user_id: &crate::domain::value_objects::UserId) -> Result<String, DomainError> {
        Ok("$argon2id$v=19$m=65536,t=3,p=4$test".to_string())
    }

    fn verify_password(password: &str, hash: &str) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| DomainError::ValidationError(format!("Invalid password hash: {}", e)))?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    fn generate_tokens(user_id: &crate::domain::value_objects::UserId) -> Result<TokenResult, DomainError> {
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

pub struct LoginResult {
    pub user: crate::domain::users::entities::user::User,
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

