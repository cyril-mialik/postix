use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{errors::Error, follow::Follow, user::{User, UserError}}, repositories::{follow::FollowRepository, user::UserRepository},
};

pub struct FollowService {
    follow_repo: Arc<dyn FollowRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl FollowService {
    pub fn new(follow_repo: Arc<dyn FollowRepository>, user_repo: Arc<dyn UserRepository>) -> Self {
        Self {
            follow_repo,
            user_repo,
        }
    }

    pub async fn follow(&self, follower_id: Uuid, following_id: Uuid) -> Result<(), Error> {
        self.user_repo
            .find_by_id(following_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;

        let follow = Follow::new(follower_id, following_id)?;
        self.follow_repo.create(&follow).await?;

        self.user_repo
            .increment_following_count(follower_id, User::INCREMENT)
            .await?;
        self.user_repo
            .increment_followers_count(following_id, User::INCREMENT)
            .await?;

        Ok(())
    }

    pub async fn unfollow(&self, follower_id: Uuid, following_id: Uuid) -> Result<(), Error> {
        self.follow_repo.delete(follower_id, following_id).await?;
        self.user_repo
            .increment_following_count(follower_id, User::DECREMENT)
            .await?;
        self.user_repo
            .increment_followers_count(following_id, User::DECREMENT)
            .await?;
        Ok(())
    }

    pub async fn get_followers(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error> {
        self.follow_repo
            .find_followers(user_id, limit, offset)
            .await
    }

    pub async fn get_following(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error> {
        self.follow_repo
            .find_following(user_id, limit, offset)
            .await
    }
}
