use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        post::PostError,
        review::{Review, ReviewError},
        user::UserError,
    },
    repositories::{post::PostRepository, review::ReviewRepository, user::UserRepository},
};

#[derive(Clone)]
pub struct ReviewService {
    review_repo: Arc<dyn ReviewRepository>,
    post_repo: Arc<dyn PostRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl ReviewService {
    pub fn new(
        review_repo: Arc<dyn ReviewRepository>,
        post_repo: Arc<dyn PostRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            review_repo,
            post_repo,
            user_repo,
        }
    }

    pub async fn create_review(
        &self,
        post_id: Uuid,
        auth_id: Uuid,
        review: String,
    ) -> Result<Review, Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;
        post.ensure_visible().map_err(Error::Post)?;

        let profile_id = self
            .user_repo
            .find_profile_id_by_auth_id(auth_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;

        let existing: Option<Review> = self
            .review_repo
            .find_by_post_and_profile(post_id, profile_id)
            .await?;

        if existing.is_some() {
            return Err(Error::Review(ReviewError::AlreadyExists));
        }

        let review = Review::new(post_id, profile_id, review)?;
        self.review_repo.create(&review).await
    }

    pub async fn get_reviews(
        &self,
        post_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Review>, Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;
        post.ensure_visible().map_err(Error::Post)?;

        self.review_repo.find_by_post(post_id, limit, offset).await
    }

    pub async fn profile_id_for_auth(&self, auth_id: Uuid) -> Result<Uuid, Error> {
        self.user_repo
            .find_profile_id_by_auth_id(auth_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))
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
        Ok(())
    }
}
