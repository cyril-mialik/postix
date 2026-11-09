use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        notification::{Notification, NotificationError},
        notification_type::NotificationType,
    },
    repositories::notification::NotificationRepository,
};

pub struct PostgresNotificationRepository {
    pool: PgPool,
}

impl PostgresNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn create(&self, n: &Notification) -> Result<Notification, Error> {
        sqlx::query!(
            r#"
            INSERT INTO notification (id, recipient_id, actor_id, post_id, notification_type_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            n.id, n.recipient_id, n.actor_id, n.post_id, n.notification_type.as_id(), n.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(n.clone())
    }

    async fn find_by_recipient(
        &self,
        recipient_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, is_read, recipient_id, actor_id, post_id, notification_type_id, created_at
            FROM notification
            WHERE recipient_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            recipient_id, limit, offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(rows
            .into_iter()
            .map(|r| Notification {
                id: r.id,
                is_read: r.is_read,
                recipient_id: r.recipient_id,
                actor_id: r.actor_id,
                post_id: r.post_id,
                notification_type: NotificationType::from_id(r.notification_type_id)
                    .unwrap_or(NotificationType::System),
                created_at: r.created_at,
            })
            .collect())
    }

    async fn mark_read(&self, id: Uuid, recipient_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!(
            "UPDATE notification SET is_read = true WHERE id = $1 AND recipient_id = $2",
            id, recipient_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::Notification(NotificationError::NotFound));
        }

        Ok(())
    }

    async fn mark_all_read(&self, recipient_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE notification SET is_read = true WHERE recipient_id = $1 AND is_read = false",
            recipient_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(())
    }

    async fn unread_count(&self, recipient_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM notification WHERE recipient_id = $1 AND is_read = false",
            recipient_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(count.unwrap_or(0))
    }
}
