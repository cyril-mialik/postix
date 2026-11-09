use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        tag::Tag,
    },
    repositories::tag::TagRepository,
};

pub struct PostgresTagRepository {
    pool: PgPool,
}

impl PostgresTagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepository for PostgresTagRepository {
    async fn find_by_name(&self, name: &str) -> Result<Option<Tag>, Error> {
        let row = sqlx::query!(
            "SELECT id, name FROM tag WHERE lower(name) = lower($1)",
            name
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(row.map(|r| Tag { id: r.id, name: r.name }))
    }

    async fn find_or_create(&self, name: &str) -> Result<Tag, Error> {
        let name = name.trim().to_lowercase();
        let row = sqlx::query!(
            r#"
            INSERT INTO tag (name) VALUES ($1)
            ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
            RETURNING id, name
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(Tag { id: row.id, name: row.name })
    }

    async fn find_by_post(&self, post_id: Uuid) -> Result<Vec<Tag>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT t.id, t.name FROM tag t
            JOIN post_tag pt ON pt.tag_id = t.id
            WHERE pt.post_id = $1
            "#,
            post_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(rows
            .into_iter()
            .map(|r| Tag { id: r.id, name: r.name })
            .collect())
    }

    async fn attach_to_post(&self, post_id: Uuid, tag_id: i32) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO post_tag (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            post_id, tag_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn detach_from_post(&self, post_id: Uuid, tag_id: i32) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM post_tag WHERE post_id = $1 AND tag_id = $2",
            post_id, tag_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }
}
