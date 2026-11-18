use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        post::PostError,
        review::ReviewError,
        review_reaction::{ReviewReaction, ReviewReactionEntity, ReviewReactionSummary},
        user::UserError,
    },
    repositories::{
        post::PostRepository, review::ReviewRepository, review_reaction::ReviewReactionRepository,
        user::UserRepository,
    },
};

#[derive(Clone)]
pub struct ReviewReactionService {
    reaction_repo: Arc<dyn ReviewReactionRepository>,
    review_repo: Arc<dyn ReviewRepository>,
    post_repo: Arc<dyn PostRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl ReviewReactionService {
    pub fn new(
        reaction_repo: Arc<dyn ReviewReactionRepository>,
        review_repo: Arc<dyn ReviewRepository>,
        post_repo: Arc<dyn PostRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            reaction_repo,
            review_repo,
            post_repo,
            user_repo,
        }
    }

    pub async fn react_to_review(
        &self,
        post_id: Uuid,
        review_id: i32,
        auth_id: Uuid,
        reaction: ReviewReaction,
    ) -> Result<ReviewReactionSummary, Error> {
        let review = self
            .review_repo
            .find_by_id(review_id)
            .await?
            .filter(|review| review.post_id == post_id)
            .ok_or(Error::Review(ReviewError::NotFound))?;

        let post = self
            .post_repo
            .find_by_id(review.post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;
        post.ensure_visible().map_err(Error::Post)?;

        let profile_id = self
            .user_repo
            .find_profile_id_by_auth_id(auth_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;
        let reaction = ReviewReactionEntity::new(review_id, profile_id, reaction);
        self.reaction_repo.upsert(&reaction).await?;

        self.reaction_repo.summary(review_id, profile_id).await
    }

    pub async fn summary(
        &self,
        review_id: i32,
        auth_id: Uuid,
    ) -> Result<ReviewReactionSummary, Error> {
        let profile_id = self
            .user_repo
            .find_profile_id_by_auth_id(auth_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;

        self.reaction_repo.summary(review_id, profile_id).await
    }
}
