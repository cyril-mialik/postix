use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        rating::{Rating, RatingError},
    },
    repositories::rating::RatingRepository,
};

pub struct PostgresRatingRepository {
    pool: PgPool,
}

impl PostgresRatingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RatingRepository for PostgresRatingRepository {
    async fn find_by_post_and_profile(
        &self,
        post_id: Uuid,
        profile_id: Uuid,
    ) -> Result<Option<Rating>, Error> {
        let row = sqlx::query!(
            r#"SELECT id, post_id, profile_id, rating, created_at, updated_at
               FROM post_rating WHERE post_id = $1 AND profile_id = $2"#,
            post_id, profile_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(row.map(|r| Rating {
            id: r.id,
            post_id: r.post_id,
            profile_id: r.profile_id,
            value: r.rating,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn create(&self, rating: &Rating) -> Result<Rating, Error> {
        let row = sqlx::query!(
            r#"INSERT INTO post_rating (post_id, profile_id, rating)
               VALUES ($1, $2, $3) RETURNING id"#,
            rating.post_id, rating.profile_id, rating.value
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                Error::Rating(RatingError::AlreadyExists)
            }
            _ => Error::System(SystemError::Database(e.to_string())),
        })?;

        Ok(Rating { id: row.id, ..rating.clone() })
    }

    async fn update(&self, rating: &Rating) -> Result<Rating, Error> {
        sqlx::query!(
            "UPDATE post_rating SET rating = $3, updated_at = NOW() WHERE post_id = $1 AND profile_id = $2",
            rating.post_id, rating.profile_id, rating.value
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(rating.clone())
    }

    async fn delete(&self, post_id: Uuid, profile_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!(
            "DELETE FROM post_rating WHERE post_id = $1 AND profile_id = $2",
            post_id, profile_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::Rating(RatingError::NotFound));
        }
        Ok(())
    }

    async fn average_for_post(&self, post_id: Uuid) -> Result<f64, Error> {
        let avg = sqlx::query_scalar!(
            r#"SELECT AVG(rating) as "avg: f64" FROM post_rating WHERE post_id = $1"#,
            post_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(avg.unwrap_or(0.0))
    }
}
