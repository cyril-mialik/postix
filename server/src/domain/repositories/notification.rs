use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, notification::Notification};

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(&self, notification: &Notification) -> Result<Notification, Error>;
    async fn find_by_recipient(
        &self,
        recipient_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, Error>;
    async fn mark_read(&self, id: Uuid, recipient_id: Uuid) -> Result<(), Error>;
    async fn mark_all_read(&self, recipient_id: Uuid) -> Result<(), Error>;
    async fn unread_count(&self, recipient_id: Uuid) -> Result<i64, Error>;
}
