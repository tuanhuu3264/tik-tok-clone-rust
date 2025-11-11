use crate::domain::value_objects::UserId;
use chrono::{DateTime, Utc};

/// Credentials Entity - User authentication credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    pub user_id: UserId,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub access_token_expires_at: DateTime<Utc>,
    pub refresh_token_expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Credentials {
    /// Create new credentials for a user
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            access_token: None,
            refresh_token: None,
            access_token_expires_at: now,
            refresh_token_expires_at: now,
            is_active: true,
            ip_address: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create credentials with tokens
    pub fn new_with_tokens(
        user_id: UserId,
        access_token: String,
        refresh_token: String,
        access_token_expires_at: DateTime<Utc>,
        refresh_token_expires_at: DateTime<Utc>,
        ip_address: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            access_token_expires_at,
            refresh_token_expires_at,
            is_active: true,
            ip_address,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update access token
    pub fn update_access_token(&mut self, token: String, expires_at: DateTime<Utc>) {
        self.access_token = Some(token);
        self.access_token_expires_at = expires_at;
        self.updated_at = Utc::now();
    }

    /// Update refresh token
    pub fn update_refresh_token(&mut self, token: String, expires_at: DateTime<Utc>) {
        self.refresh_token = Some(token);
        self.refresh_token_expires_at = expires_at;
        self.updated_at = Utc::now();
    }

    /// Update both tokens
    pub fn update_tokens(
        &mut self,
        access_token: String,
        refresh_token: String,
        access_token_expires_at: DateTime<Utc>,
        refresh_token_expires_at: DateTime<Utc>,
    ) {
        self.access_token = Some(access_token);
        self.refresh_token = Some(refresh_token);
        self.access_token_expires_at = access_token_expires_at;
        self.refresh_token_expires_at = refresh_token_expires_at;
        self.updated_at = Utc::now();
    }

    /// Update IP address
    pub fn update_ip_address(&mut self, ip_address: Option<String>) {
        self.ip_address = ip_address;
        self.updated_at = Utc::now();
    }

    /// Deactivate credentials
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Activate credentials
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Check if access token is expired
    pub fn is_access_token_expired(&self) -> bool {
        Utc::now() > self.access_token_expires_at
    }

    /// Check if refresh token is expired
    pub fn is_refresh_token_expired(&self) -> bool {
        Utc::now() > self.refresh_token_expires_at
    }

    /// Check if credentials are valid (active and tokens not expired)
    pub fn is_valid(&self) -> bool {
        self.is_active
            && !self.is_access_token_expired()
            && !self.is_refresh_token_expired()
    }

    /// Revoke credentials (clear tokens and deactivate)
    pub fn revoke(&mut self) {
        self.access_token = None;
        self.refresh_token = None;
        self.is_active = false;
        self.updated_at = Utc::now();
    }
}
