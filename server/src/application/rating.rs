use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        post::PostError,
        rating::{Rating, RatingError},
    },
    repositories::{post::PostRepository, rating::RatingRepository},
};

pub struct RatingService {
    rating_repo: Arc<dyn RatingRepository>,
    post_repo: Arc<dyn PostRepository>,
}

impl RatingService {
    pub fn new(rating_repo: Arc<dyn RatingRepository>, post_repo: Arc<dyn PostRepository>) -> Self {
        Self {
            rating_repo,
            post_repo,
        }
    }

    pub async fn rate_post(
        &self,
        post_id: Uuid,
        profile_id: Uuid,
        value: i32,
    ) -> Result<Rating, Error> {
        let _post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;

        if let Some(existing) = self
            .rating_repo
            .find_by_post_and_profile(post_id, profile_id)
            .await?
        {
            let old_value = existing.value;
            let updated = Rating {
                value,
                updated_at: chrono::Utc::now(),
                ..existing
            };
            let result = self.rating_repo.update(&updated).await?;

            self.post_repo
                .update_rating_stats(post_id, value - old_value, 0)
                .await?;

            return Ok(result);
        }

        let rating = Rating::new(post_id, profile_id, value)?; // ← проверка здесь
        let created = self.rating_repo.create(&rating).await?;

        self.post_repo
            .update_rating_stats(post_id, value, 1)
            .await?;

        Ok(created)
    }

    pub async fn remove_rating(&self, post_id: Uuid, profile_id: Uuid) -> Result<(), Error> {
        let existing = self
            .rating_repo
            .find_by_post_and_profile(post_id, profile_id)
            .await?
            .ok_or(Error::Rating(RatingError::NotFound))?;

        self.rating_repo.delete(post_id, profile_id).await?;
        self.post_repo
            .update_rating_stats(post_id, -existing.value, -1)
            .await?;

        Ok(())
    }
}
