use crate::domain::entities::{errors::Error, theme::Theme};
use async_trait::async_trait;

#[async_trait]
pub trait ThemeRepository: Send + Sync {
    async fn find_by_name(&self, name: String) -> Result<Option<Theme>, Error>;

    async fn find_all(&self) -> Result<Vec<Theme>, Error>;
}
