use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, view::PostView};

#[async_trait]
pub trait ViewRepository: Send + Sync {
    async fn record_view(&self, view: &PostView) -> Result<(), Error>;
    async fn count_for_post(&self, post_id: Uuid) -> Result<i64, Error>;
}
