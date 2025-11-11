use crate::domain::value_objects::UserId;
use chrono::{DateTime, Utc};
use crate::domain::value_objects::StatusId;

#[derive(Debug, Clone)]
pub struct Profile {
    pub user_id: UserId,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status_id : StatusId
}

impl Profile {
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            display_name: None,
            bio: None,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_with_details(
        user_id: UserId,
        display_name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            display_name,
            bio,
            avatar_url,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_display_name(&mut self, display_name: Option<String>) {
        self.display_name = display_name;
        self.updated_at = Utc::now();
    }

    pub fn update_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
        self.updated_at = Utc::now();
    }

    pub fn update_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
        self.updated_at = Utc::now();
    }

    pub fn update_profile(
        &mut self,
        display_name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) {
        self.display_name = display_name;
        self.bio = bio;
        self.avatar_url = avatar_url;
        self.updated_at = Utc::now();
    }
}

