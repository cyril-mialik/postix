use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::errors::Error;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BookmarkRepository: Send + Sync {
    async fn exists(&self, user_id: Uuid, post_id: Uuid) -> Result<bool, Error>;
    async fn create(&self, user_id: Uuid, post_id: Uuid) -> Result<(), Error>;
    async fn delete(&self, user_id: Uuid, post_id: Uuid) -> Result<(), Error>;
    async fn find_by_profile(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error>;
}
