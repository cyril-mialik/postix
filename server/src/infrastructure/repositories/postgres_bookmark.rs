use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        bookmark::Bookmark,
    },
    repositories::bookmark::BookmarkRepository,
};

pub struct PostgresBookmarkRepository {
    pool: PgPool,
}

impl PostgresBookmarkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookmarkRepository for PostgresBookmarkRepository {
    async fn exists(&self, profile_id: Uuid, post_id: Uuid) -> Result<bool, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM post_bookmark WHERE profile_id = $1 AND post_id = $2) as "exists!""#,
            profile_id, post_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(exists)
    }

    async fn create(&self, bookmark: &Bookmark) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO post_bookmark (profile_id, post_id) VALUES ($1, $2)",
            bookmark.profile_id, bookmark.post_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::System(SystemError::Database("already bookmarked".into()))
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;
        Ok(())
    }

    async fn delete(&self, profile_id: Uuid, post_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!(
            "DELETE FROM post_bookmark WHERE profile_id = $1 AND post_id = $2",
            profile_id, post_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::System(SystemError::Database("not bookmarked".into())));
        }
        Ok(())
    }

    async fn find_by_profile(
        &self,
        profile_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error> {
        let rows = sqlx::query_scalar!(
            "SELECT post_id FROM post_bookmark WHERE profile_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            profile_id, limit, offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(rows)
    }
}
