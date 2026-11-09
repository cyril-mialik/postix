use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        view::PostView,
    },
    repositories::view::ViewRepository,
};

pub struct PostgresViewRepository {
    pool: PgPool,
}

impl PostgresViewRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ViewRepository for PostgresViewRepository {
    async fn record_view(&self, view: &PostView) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO post_view (post_id, viewer_id, viewed_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (post_id, viewer_id, viewed_at) DO NOTHING
            "#,
            view.post_id, view.viewer_id, view.viewed_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn count_for_post(&self, post_id: Uuid) -> Result<i64, Error> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM post_view WHERE post_id = $1",
            post_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(count.unwrap_or(0))
    }
}
