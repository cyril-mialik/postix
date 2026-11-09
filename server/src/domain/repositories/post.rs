use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, post::Post};

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Error>;
    async fn find_feed(&self, limit: i64, offset: i64) -> Result<Vec<Post>, Error>;
    async fn find_by_author(
        &self,
        author_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, Error>;
    async fn create(&self, post: &Post) -> Result<Post, Error>;
    async fn update(&self, post: &Post) -> Result<Post, Error>;
    async fn soft_delete(&self, id: Uuid, deleted_by: Uuid) -> Result<(), Error>;
    async fn increment_views(&self, id: Uuid) -> Result<(), Error>;
    async fn update_rating_stats(
        &self,
        id: Uuid,
        rating_sum_delta: i32,
        rating_count_delta: i32,
    ) -> Result<(), Error>;
    async fn update_reviews_count(&self, id: Uuid, delta: i32) -> Result<(), Error>;
}
