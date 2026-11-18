use async_trait::async_trait;

use crate::domain::entities::{
    errors::Error,
    review_reaction::{ReviewReactionEntity, ReviewReactionSummary},
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ReviewReactionRepository: Send + Sync {
    async fn upsert(&self, reaction: &ReviewReactionEntity) -> Result<ReviewReactionEntity, Error>;
    async fn summary(
        &self,
        review_id: i32,
        profile_id: uuid::Uuid,
    ) -> Result<ReviewReactionSummary, Error>;
}
