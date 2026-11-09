use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, follow::Follow};

#[async_trait]
pub trait FollowRepository: Send + Sync {
    async fn exists(&self, follower_id: Uuid, following_id: Uuid) -> Result<bool, Error>;
    async fn create(&self, follow: &Follow) -> Result<(), Error>;
    async fn delete(&self, follower_id: Uuid, following_id: Uuid) -> Result<(), Error>;
    async fn find_followers(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error>;
    async fn find_following(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error>;
}
