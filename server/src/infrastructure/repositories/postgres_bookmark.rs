use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::errors::{Error, SystemError},
    repositories::bookmark::BookmarkRepository,
};

pub struct PostgresBookmarkRepository {
    pool: PgPool,
}

impl PostgresBookmarkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn profile_id(&self, user_id: Uuid) -> Result<Uuid, Error> {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM profile WHERE auth_id = $1 AND deleted_at IS NULL",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or_else(|| Error::System(SystemError::Database("profile not found".to_owned())))
    }
}

#[async_trait]
impl BookmarkRepository for PostgresBookmarkRepository {
    async fn exists(&self, user_id: Uuid, post_id: Uuid) -> Result<bool, Error> {
        let profile_id = self.profile_id(user_id).await?;
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM post_bookmark WHERE profile_id = $1 AND post_id = $2) as "exists!""#,
            profile_id, post_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(exists)
    }

    async fn create(&self, user_id: Uuid, post_id: Uuid) -> Result<(), Error> {
        let profile_id = self.profile_id(user_id).await?;
        sqlx::query(
            "INSERT INTO post_bookmark (profile_id, post_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(profile_id)
        .bind(post_id)
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

    async fn delete(&self, user_id: Uuid, post_id: Uuid) -> Result<(), Error> {
        let profile_id = self.profile_id(user_id).await?;
        sqlx::query!(
            "DELETE FROM post_bookmark WHERE profile_id = $1 AND post_id = $2",
            profile_id,
            post_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(())
    }

    async fn find_by_profile(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error> {
        let profile_id = self.profile_id(user_id).await?;
        let rows = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT bookmark.post_id
            FROM post_bookmark bookmark
            JOIN post ON post.id = bookmark.post_id
            WHERE bookmark.profile_id = $1 AND post.status_id = 2 AND post.deleted_at IS NULL
            ORDER BY bookmark.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(profile_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(rows)
    }
}
