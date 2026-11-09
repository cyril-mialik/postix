use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{errors::Error, notification::Notification},
    repositories::notification::NotificationRepository,
};

pub struct NotificationService {
    repo: Arc<dyn NotificationRepository>,
}

impl NotificationService {
    pub fn new(repo: Arc<dyn NotificationRepository>) -> Self {
        Self { repo }
    }

    pub async fn list(
        &self,
        recipient_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, Error> {
        self.repo.find_by_recipient(recipient_id, limit, offset).await
    }

    pub async fn mark_read(&self, id: Uuid, recipient_id: Uuid) -> Result<(), Error> {
        self.repo.mark_read(id, recipient_id).await
    }

    pub async fn mark_all_read(&self, recipient_id: Uuid) -> Result<(), Error> {
        self.repo.mark_all_read(recipient_id).await
    }

    pub async fn unread_count(&self, recipient_id: Uuid) -> Result<i64, Error> {
        self.repo.unread_count(recipient_id).await
    }
}
