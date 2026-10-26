use crate::domain::entities::{errors::Error, post::Post};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, user: &Post) -> Result<Post, Error>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Error>;

    async fn update_title(&self, id: Uuid, new_title: &str) -> Result<Post, Error>;

    async fn update_post(
        &self,
        id: Uuid,
        new_title: &str,
        new_content: &str,
    ) -> Result<Post, Error>;

    async fn update_content(&self, id: Uuid, new_content: &str) -> Result<Post, Error>;

    async fn delete_post(&self, id: Uuid) -> Result<(), Error>;
}
