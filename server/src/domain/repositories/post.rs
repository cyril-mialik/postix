use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, post::Post};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Error>;
    async fn find_feed(&self, limit: i64, offset: i64) -> Result<Vec<Post>, Error>;
    async fn find_by_author(
        &self,
        author_id: Uuid,
        status_id: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, Error>;
    async fn create(&self, post: &Post) -> Result<Post, Error>;
    async fn update(&self, post: &Post) -> Result<Post, Error>;
    async fn soft_delete(&self, id: Uuid, deleted_by: Uuid) -> Result<(), Error>;
    async fn increment_views(&self, id: Uuid) -> Result<(), Error>;
    async fn total_views_by_author(&self, author_id: Uuid) -> Result<i64, Error>;
    async fn update_reviews_count(&self, id: Uuid, delta: i32) -> Result<(), Error>;
}
