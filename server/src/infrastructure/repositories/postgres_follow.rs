use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        follow::Follow,
    },
    repositories::follow::FollowRepository,
};

pub struct PostgresFollowRepository {
    pool: PgPool,
}

impl PostgresFollowRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FollowRepository for PostgresFollowRepository {
    async fn exists(&self, follower_id: Uuid, following_id: Uuid) -> Result<bool, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM follow WHERE follower_id = $1 AND following_id = $2) as "exists!""#,
            follower_id, following_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(exists)
    }

    async fn create(&self, follow: &Follow) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO follow (follower_id, following_id) VALUES ($1, $2)",
            follow.follower_id, follow.following_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::System(SystemError::Database("already following".into()))
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;
        Ok(())
    }

    async fn delete(&self, follower_id: Uuid, following_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!(
            "DELETE FROM follow WHERE follower_id = $1 AND following_id = $2",
            follower_id, following_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::System(SystemError::Database("not following".into())));
        }
        Ok(())
    }

    async fn find_followers(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error> {
        let rows = sqlx::query_scalar!(
            "SELECT follower_id FROM follow WHERE following_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            user_id, limit, offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(rows)
    }

    async fn find_following(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error> {
        let rows = sqlx::query_scalar!(
            "SELECT following_id FROM follow WHERE follower_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            user_id, limit, offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(rows)
    }
}
