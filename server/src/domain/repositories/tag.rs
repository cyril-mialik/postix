use crate::domain::entities::{errors::Error, tag::Tag};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TagRepository: Send + Sync {
    async fn find_by_name(&self, name: &str) -> Result<Option<Tag>, Error>;
    async fn find_or_create(&self, name: &str) -> Result<Tag, Error>;
    async fn find_by_post(&self, post_id: Uuid) -> Result<Vec<Tag>, Error>;
    async fn attach_to_post(&self, post_id: Uuid, tag_id: i32) -> Result<(), Error>;
    async fn detach_from_post(&self, post_id: Uuid, tag_id: i32) -> Result<(), Error>;
}
