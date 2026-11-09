use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        review::{Review, ReviewError},
    },
    repositories::review::ReviewRepository,
};

pub struct PostgresReviewRepository {
    pool: PgPool,
}

impl PostgresReviewRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewRepository for PostgresReviewRepository {
    async fn find_by_post_and_profile(
        &self,
        post_id: Uuid,
        profile_id: Uuid,
    ) -> Result<Option<Review>, Error> {
        let row = sqlx::query!(
            r#"SELECT id, post_id, profile_id, review, created_at, updated_at
               FROM post_review WHERE post_id = $1 AND profile_id = $2"#,
            post_id, profile_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(row.map(|r| Review {
            id: r.id,
            post_id: r.post_id,
            profile_id: r.profile_id,
            review: r.review,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn find_by_post(
        &self,
        post_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Review>, Error> {
        let rows = sqlx::query!(
            r#"SELECT id, post_id, profile_id, review, created_at, updated_at
               FROM post_review WHERE post_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"#,
            post_id, limit, offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(rows
            .into_iter()
            .map(|r| Review {
                id: r.id,
                post_id: r.post_id,
                profile_id: r.profile_id,
                review: r.review,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    async fn create(&self, review: &Review) -> Result<Review, Error> {
        let row = sqlx::query!(
            r#"INSERT INTO post_review (post_id, profile_id, review)
               VALUES ($1, $2, $3) RETURNING id"#,
            review.post_id, review.profile_id, review.review
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::Review(ReviewError::AlreadyExists)
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;

        Ok(Review { id: row.id, ..review.clone() })
    }

    async fn update(&self, review: &Review) -> Result<Review, Error> {
        sqlx::query!(
            "UPDATE post_review SET review = $3, updated_at = NOW() WHERE post_id = $1 AND profile_id = $2",
            review.post_id, review.profile_id, review.review
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(review.clone())
    }

    async fn delete(&self, post_id: Uuid, profile_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!(
            "DELETE FROM post_review WHERE post_id = $1 AND profile_id = $2",
            post_id, profile_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::Review(ReviewError::NotFound));
        }
        Ok(())
    }
}
