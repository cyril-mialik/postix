use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{bookmark::Bookmark, errors::Error};

#[async_trait]
pub trait BookmarkRepository: Send + Sync {
    async fn exists(&self, profile_id: Uuid, post_id: Uuid) -> Result<bool, Error>;
    async fn create(&self, bookmark: &Bookmark) -> Result<(), Error>;
    async fn delete(&self, profile_id: Uuid, post_id: Uuid) -> Result<(), Error>;
    async fn find_by_profile(
        &self,
        profile_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error>;
}
