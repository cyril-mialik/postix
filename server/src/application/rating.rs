use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        post::PostError,
        rating::{Rating, RatingError},
        user::UserError,
    },
    repositories::{post::PostRepository, rating::RatingRepository, user::UserRepository},
};
use postix_core::post::PostRatingSummary;

#[derive(Clone)]
pub struct RatingService {
    rating_repo: Arc<dyn RatingRepository>,
    post_repo: Arc<dyn PostRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl RatingService {
    pub fn new(
        rating_repo: Arc<dyn RatingRepository>,
        post_repo: Arc<dyn PostRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            rating_repo,
            post_repo,
            user_repo,
        }
    }

    pub async fn rate_post(
        &self,
        post_id: Uuid,
        auth_id: Uuid,
        value: i32,
    ) -> Result<PostRatingSummary, Error> {
        Rating::check_value(value)?;
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;
        post.ensure_visible().map_err(Error::Post)?;
        let profile_id = self.profile_id_for_auth_id(auth_id).await?;

        if let Some(existing) = self
            .rating_repo
            .find_by_post_and_profile(post_id, profile_id)
            .await?
        {
            let updated = Rating {
                value,
                updated_at: chrono::Utc::now(),
                ..existing
            };
            self.rating_repo.update(&updated).await?;
        } else {
            let rating = Rating::new(post_id, profile_id, value)?;
            self.rating_repo.create(&rating).await?;
        }

        self.summary_for_profile(post_id, profile_id).await
    }

    pub async fn summary(&self, post_id: Uuid, auth_id: Uuid) -> Result<PostRatingSummary, Error> {
        let profile_id = self.profile_id_for_auth_id(auth_id).await?;
        self.summary_for_profile(post_id, profile_id).await
    }

    async fn profile_id_for_auth_id(&self, auth_id: Uuid) -> Result<Uuid, Error> {
        self.user_repo
            .find_profile_id_by_auth_id(auth_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))
    }

    async fn summary_for_profile(
        &self,
        post_id: Uuid,
        profile_id: Uuid,
    ) -> Result<PostRatingSummary, Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;
        post.ensure_visible().map_err(Error::Post)?;

        let my_rating = self
            .rating_repo
            .find_by_post_and_profile(post_id, profile_id)
            .await?
            .map(|rating| rating.value);

        Ok(PostRatingSummary {
            rating_sum: post.rating_sum,
            rating_count: post.rating_count,
            my_rating,
        })
    }

    pub async fn remove_rating(&self, post_id: Uuid, profile_id: Uuid) -> Result<(), Error> {
        self.rating_repo
            .find_by_post_and_profile(post_id, profile_id)
            .await?
            .ok_or(Error::Rating(RatingError::NotFound))?;

        self.rating_repo.delete(post_id, profile_id).await
    }
}
