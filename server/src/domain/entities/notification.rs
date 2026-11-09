use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use super::notification_type::NotificationType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub is_read: bool,
    pub recipient_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub post_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub created_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(
        recipient_id: Uuid,
        actor_id: Option<Uuid>,
        post_id: Option<Uuid>,
        notification_type: NotificationType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            is_read: false,
            recipient_id,
            actor_id,
            post_id,
            notification_type,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Error)]
pub enum NotificationError {
    #[error("Уведомление не найдено")]
    NotFound,
}
