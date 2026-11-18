use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        media_type::MediaType,
        post::{Post, PostMedia},
        post_status::PostStatus,
        tag::Tag,
        user::UserError,
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

    async fn load_media(&self, post_id: Uuid) -> Result<Vec<PostMedia>, Error> {
        let rows = sqlx::query!(
            r#"SELECT id, url, media_type, position FROM post_media WHERE post_id = $1 ORDER BY position"#,
            post_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(rows
            .into_iter()
            .map(|r| PostMedia {
                id: r.id,
                url: r.url,
                media_type: match r.media_type.as_str() {
                    "image" => MediaType::Image,
                    "video" => MediaType::Video,
                    _ => MediaType::Image,
                },
                position: r.position,
            })
            .collect())
    }

    async fn load_tags(&self, post_id: Uuid) -> Result<Vec<Tag>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT t.id, t.name
            FROM tag t
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
            .map(|r| Tag {
                id: r.id,
                name: r.name,
            })
            .collect())
    }

    async fn build_post(&self, row: PostRow) -> Result<Post, Error> {
        let media = self.load_media(row.id).await?;
        let tags = self.load_tags(row.id).await?;
        Ok(Post {
            id: row.id,
            author_id: row.author_id,
            title: row.title,
            content: row.content,
            status: PostStatus::from_id(row.status_id).unwrap_or(PostStatus::Draft),
            rating_sum: row.rating_sum,
            rating_count: row.rating_count,
            reviews_count: row.reviews_count,
            views_count: row.views_count,
            media,
            tags,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
            deleted_by: row.deleted_by,
        })
    }
}

#[derive(sqlx::FromRow)]
struct PostRow {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: serde_json::Value,
    pub status_id: i32,
    pub rating_sum: i32,
    pub rating_count: i32,
    pub reviews_count: i32,
    pub views_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Error> {
        let row = sqlx::query_as::<_, PostRow>(
            r#"
            SELECT p.id, profile.auth_id AS author_id, p.title, p.content, p.status_id,
                   p.rating_sum, p.rating_count, p.reviews_count, p.views_count,
                   p.created_at, p.updated_at, p.deleted_at, p.deleted_by
            FROM post p
            JOIN profile ON profile.id = p.profile_id
            WHERE p.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        match row {
            Some(r) => Ok(Some(self.build_post(r).await?)),
            None => Ok(None),
        }
    }

    async fn find_feed(&self, limit: i64, offset: i64) -> Result<Vec<Post>, Error> {
        let rows = sqlx::query_as::<_, PostRow>(
            r#"
            SELECT p.id, profile.auth_id AS author_id, p.title, p.content, p.status_id,
                   p.rating_sum, p.rating_count, p.reviews_count, p.views_count,
                   p.created_at, p.updated_at, p.deleted_at, p.deleted_by
            FROM post p
            JOIN profile ON profile.id = p.profile_id
            WHERE p.status_id = 2 AND p.deleted_at IS NULL
            ORDER BY p.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        let mut posts = Vec::with_capacity(rows.len());
        for r in rows {
            posts.push(self.build_post(r).await?);
        }
        Ok(posts)
    }

    async fn find_by_author(
        &self,
        author_id: Uuid,
        status_id: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, Error> {
        let rows = sqlx::query_as::<_, PostRow>(
            r#"
            SELECT p.id, profile.auth_id AS author_id, p.title, p.content, p.status_id,
                   p.rating_sum, p.rating_count, p.reviews_count, p.views_count,
                   p.created_at, p.updated_at, p.deleted_at, p.deleted_by
            FROM post p
            JOIN profile ON profile.id = p.profile_id
            WHERE profile.auth_id = $1 AND p.status_id = $2 AND p.deleted_at IS NULL
            ORDER BY p.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(author_id)
        .bind(status_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        let mut posts = Vec::with_capacity(rows.len());
        for r in rows {
            posts.push(self.build_post(r).await?);
        }
        Ok(posts)
    }

    async fn create(&self, post: &Post) -> Result<Post, Error> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        let profile_id = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM profile WHERE auth_id = $1 AND deleted_at IS NULL",
        )
        .bind(post.author_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::User(UserError::NotFound))?;

        sqlx::query!(
            r#"
            INSERT INTO post (id, profile_id, title, content, status_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            post.id,
            profile_id,
            post.title,
            post.content,
            post.status.as_id(),
            post.created_at,
            post.updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        for (idx, media) in post.media.iter().enumerate() {
            sqlx::query!(
                "INSERT INTO post_media (id, post_id, url, media_type, position) VALUES ($1, $2, $3, $4, $5)",
                media.id, post.id, media.url, media.media_type.as_str(), idx as i32
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        }

        for tag in &post.tags {
            sqlx::query!(
                "INSERT INTO post_tag (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                post.id,
                tag.id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        }

        tx.commit()
            .await
            .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(post.clone())
    }

    async fn update(&self, post: &Post) -> Result<Post, Error> {
        sqlx::query!(
            "UPDATE post SET title = $2, content = $3, status_id = $4, updated_at = NOW() WHERE id = $1",
            post.id, post.title, post.content, post.status.as_id()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(post.clone())
    }

    async fn soft_delete(&self, id: Uuid, deleted_by: Uuid) -> Result<(), Error> {
        let profile_id = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM profile WHERE auth_id = $1 AND deleted_at IS NULL",
        )
        .bind(deleted_by)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::User(UserError::NotFound))?;

        sqlx::query(
            "UPDATE post SET deleted_at = NOW(), deleted_by = $2, status_id = 4 WHERE id = $1",
        )
        .bind(id)
        .bind(profile_id)
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn increment_views(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE post SET views_count = views_count + 1 WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }

    async fn update_reviews_count(&self, id: Uuid, delta: i32) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE post SET reviews_count = reviews_count + $2 WHERE id = $1",
            id,
            delta
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;
        Ok(())
    }
}
