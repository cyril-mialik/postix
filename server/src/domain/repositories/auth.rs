use crate::domain::entities::errors::Error;
use async_trait::async_trait;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_email(&self, email: String) -> Result<Option<Auth>, Error>;
}
