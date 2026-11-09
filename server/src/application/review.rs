use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        post::PostError,
        review::{Review, ReviewError},
    },
    repositories::{post::PostRepository, review::ReviewRepository},
};

pub struct ReviewService {
    review_repo: Arc<dyn ReviewRepository>,
    post_repo: Arc<dyn PostRepository>,
}

impl ReviewService {
    pub fn new(review_repo: Arc<dyn ReviewRepository>, post_repo: Arc<dyn PostRepository>) -> Self {
        Self { review_repo, post_repo }
    }

    pub async fn create_review(
        &self,
        post_id: Uuid,
        profile_id: Uuid,
        review: String,
    ) -> Result<Review, Error> {
        let _post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;

        let existing: Option<Review> = self
            .review_repo
            .find_by_post_and_profile(post_id, profile_id)
            .await?;

        if existing.is_some() {
            return Err(Error::Review(ReviewError::AlreadyExists));
        }

        let review = Review::new(post_id, profile_id, review)?;
        let created = self.review_repo.create(&review).await?;

        self.post_repo.update_reviews_count(post_id, 1).await?;

        Ok(created)
    }

    pub async fn update_review(
        &self,
        post_id: Uuid,
        profile_id: Uuid,
        review: String,
    ) -> Result<Review, Error> {
        let existing: Option<Review> = self
            .review_repo
            .find_by_post_and_profile(post_id, profile_id)
            .await?;

        let existing = existing.ok_or(Error::Review(ReviewError::NotFound))?;

        let updated = Review {
            review: review.trim().to_owned(),
            updated_at: chrono::Utc::now(),
            ..existing
        };

        self.review_repo.update(&updated).await
    }

    pub async fn delete_review(&self, post_id: Uuid, profile_id: Uuid) -> Result<(), Error> {
        self.review_repo.delete(post_id, profile_id).await?;
        self.post_repo.update_reviews_count(post_id, -1).await?;
        Ok(())
    }
}
