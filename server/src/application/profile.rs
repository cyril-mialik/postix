use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        user::{User, UserError},
    },
    repositories::user::UserRepository,
};

#[derive(Clone)]
pub struct ProfileService {
    user_repo: Arc<dyn UserRepository>,
}

impl ProfileService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn get_profile(&self, user_id: Uuid) -> Result<User, Error> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))
    }

    pub async fn update_profile(
        &self,
        user_id: Uuid,
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        avatar: Option<String>,
    ) -> Result<User, Error> {
        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;

        if let Some(username) = username {
            let username = username.trim().to_lowercase();
            if !(3..=128).contains(&username.chars().count()) {
                return Err(Error::User(UserError::InvalidUsernameLength));
            }
            user.username = username;
        }
        if let Some(first_name) = first_name {
            let first_name = first_name.trim();
            user.first_name = if first_name.is_empty() {
                None
            } else {
                Some(first_name.to_owned())
            };
        }
        if let Some(last_name) = last_name {
            let last_name = last_name.trim();
            user.last_name = if last_name.is_empty() {
                None
            } else {
                Some(last_name.to_owned())
            };
        }
        if let Some(avatar) = avatar {
            let avatar = avatar.trim();
            user.avatar = if avatar.is_empty() {
                None
            } else {
                Some(avatar.to_owned())
            };
        }

        user.updated_at = chrono::Utc::now();
        self.user_repo.update(&user).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::domain::{
        entities::{role::Role, theme::Theme},
        repositories::user::MockUserRepository,
    };

    #[actix_web::test]
    async fn update_profile_normalizes_fields_and_clears_empty_optional_values() {
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            email: "reader@example.com".to_owned(),
            password_hash: "hash".to_owned(),
            is_email_verified: false,
            is_banned: false,
            ban_reason: None,
            banned_at: None,
            role: Role::User,
            username: "reader".to_owned(),
            avatar: Some("https://example.com/avatar.png".to_owned()),
            first_name: Some("Old".to_owned()),
            last_name: Some("Name".to_owned()),
            followers_count: 0,
            following_count: 0,
            posts_count: 0,
            theme: Theme::Auto,
            deleted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(user.clone())));
        user_repo
            .expect_update()
            .once()
            .returning(|user| Ok(user.clone()));

        let updated = ProfileService::new(Arc::new(user_repo))
            .update_profile(
                user_id,
                Some("  ADA  ".to_owned()),
                Some(" Ada ".to_owned()),
                Some(" ".to_owned()),
                Some(" ".to_owned()),
            )
            .await
            .unwrap();

        assert_eq!(updated.username, "ada");
        assert_eq!(updated.first_name.as_deref(), Some("Ada"));
        assert_eq!(updated.last_name, None);
        assert_eq!(updated.avatar, None);
    }
}
