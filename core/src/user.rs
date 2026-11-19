use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub avatar: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub posts_count: i32,
    pub role: String,
    pub theme: String,
    pub is_email_verified: bool,
    pub is_banned: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar: Option<String>,
}
