use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        post::{Post, PostError},
    },
    repositories::post::PostRepository,
};

pub struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create_post(&self, post: &Post) -> Result<Post, Error> {
        let created_post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, title, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO NOTHING
            RETURNING id, title, content, created_at, updated_at
            "#,
            post.id,
            post.title,
            post.content,
            post.created_at,
            post.updated_at
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Post(PostError::AlreadyExists))?;

        Ok(created_post)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, content, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(post)
    }

    async fn update_post(
        &self,
        id: Uuid,
        new_title: &str,
        new_content: &str,
    ) -> Result<Post, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1) as "exists!""#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::Post(PostError::NotFound));
        }

        let updated_post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = $2, content = $3, updated_at = $4
            WHERE id = $1
            RETURNING id, title, content, created_at, updated_at
            "#,
            id,
            new_title,
            new_content,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Post(PostError::AlreadyExists))?;

        Ok(updated_post)
    }

    async fn update_title(&self, id: Uuid, new_title: &str) -> Result<Post, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1) as "exists!""#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::Post(PostError::NotFound));
        }

        let updated_post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = $2, updated_at = $3
            WHERE id = $1
            RETURNING id, title, content, created_at, updated_at
            "#,
            id,
            new_title,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Post(PostError::AlreadyExists))?;

        Ok(updated_post)
    }

    async fn update_content(&self, id: Uuid, new_content: &str) -> Result<Post, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1) as "exists!""#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if !exists {
            return Err(Error::Post(PostError::NotFound));
        }

        let updated_content = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET content = $2, updated_at = $3
            WHERE id = $1
            RETURNING id, title, content, created_at, updated_at
            "#,
            id,
            new_content,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Post(PostError::NotFound))?;

        Ok(updated_content)
    }

    async fn delete_post(&self, id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        if result.rows_affected() == 0 {
            return Err(Error::Post(PostError::NotFound));
        }

        Ok(())
    }
}
