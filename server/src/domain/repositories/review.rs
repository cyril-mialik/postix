use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, review::Review};

#[async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn find_by_post_and_profile(
        &self,
        post_id: Uuid,
        profile_id: Uuid,
    ) -> Result<Option<Review>, Error>;

    async fn find_by_post(
        &self,
        post_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Review>, Error>;

    async fn create(&self, review: &Review) -> Result<Review, Error>;
    async fn update(&self, review: &Review) -> Result<Review, Error>;
    async fn delete(&self, post_id: Uuid, profile_id: Uuid) -> Result<(), Error>;
}
