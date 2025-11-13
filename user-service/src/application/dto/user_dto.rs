use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}


