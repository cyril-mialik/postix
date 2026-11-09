use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        follow::Follow,
        user::{User, UserError},
    },
    repositories::{follow::FollowRepository, user::UserRepository},
};

pub struct ProfileService {
    user_repo: Arc<dyn UserRepository>,
    follow_repo: Arc<dyn FollowRepository>,
}

impl ProfileService {
    pub fn new(user_repo: Arc<dyn UserRepository>, follow_repo: Arc<dyn FollowRepository>) -> Self {
        Self {
            user_repo,
            follow_repo,
        }
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
            if username.len() < 3 || username.len() > 128 {
                return Err(Error::User(UserError::InvalidUsernameLength));
            }
            user.username = username;
        }
        if let Some(first_name) = first_name {
            user.first_name = if first_name.is_empty() {
                None
            } else {
                Some(first_name)
            };
        }
        if let Some(last_name) = last_name {
            user.last_name = if last_name.is_empty() {
                None
            } else {
                Some(last_name)
            };
        }
        if let Some(avatar) = avatar {
            user.avatar = Some(avatar);
        }

        self.user_repo.update(&user).await
    }

    pub async fn follow(&self, follower_id: Uuid, following_id: Uuid) -> Result<(), Error> {
        let follow = Follow::new(follower_id, following_id)?;

        self.follow_repo.create(&follow).await?;

        self.user_repo
            .increment_following_count(follower_id, 1)
            .await?;
        self.user_repo
            .increment_followers_count(following_id, 1)
            .await?;

        Ok(())
    }

    pub async fn unfollow(&self, follower_id: Uuid, following_id: Uuid) -> Result<(), Error> {
        self.follow_repo.delete(follower_id, following_id).await?;
        self.user_repo
            .increment_following_count(follower_id, -1)
            .await?;
        self.user_repo
            .increment_followers_count(following_id, -1)
            .await?;
        Ok(())
    }
}
