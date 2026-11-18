pub use postix_core::user::{UpdateProfileRequest, UserResponse};

use crate::domain::entities::user::User;

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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;
    use crate::domain::entities::{role::Role, theme::Theme};

    #[test]
    fn user_response_exposes_profile_fields_without_credentials() {
        let user = User {
            id: Uuid::nil(),
            email: "reader@example.com".to_owned(),
            password_hash: "sensitive-hash".to_owned(),
            is_email_verified: true,
            is_banned: false,
            ban_reason: None,
            banned_at: None,
            role: Role::Moderator,
            username: "reader".to_owned(),
            avatar: Some("avatar.png".to_owned()),
            first_name: Some("Ada".to_owned()),
            last_name: Some("Lovelace".to_owned()),
            followers_count: 7,
            following_count: 9,
            posts_count: 3,
            theme: Theme::Dark,
            deleted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let response = UserResponse::from(user);

        assert_eq!(response.id, Uuid::nil());
        assert_eq!(response.email, "reader@example.com");
        assert_eq!(response.role, "moderator");
        assert_eq!(response.theme, "dark");
        assert_eq!(response.followers_count, 7);
        assert_eq!(response.following_count, 9);
        assert_eq!(response.posts_count, 3);
        assert!(response.is_email_verified);

        let json = serde_json::to_value(response).unwrap();
        assert!(json.get("password_hash").is_none());
    }
}
