use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::user::User;

#[derive(Debug, Serialize, Deserialize)]
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

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            avatar: user.avatar,
            first_name: user.first_name,
            last_name: user.last_name,
            followers_count: user.followers_count,
            following_count: user.following_count,
            posts_count: user.posts_count,
            role: user.role.as_str().to_string(),
            theme: match user.theme {
                crate::domain::entities::theme::Theme::Light => "light".to_string(),
                crate::domain::entities::theme::Theme::Dark => "dark".to_string(),
                crate::domain::entities::theme::Theme::Auto => "auto".to_string(),
            },
            is_email_verified: user.is_email_verified,
            is_banned: user.is_banned,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar: Option<String>,
}
