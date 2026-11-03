use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        post::{
            CreatePostDto, DeletePostDto, FindPostByIdDto, FindPostByUserIdDto, Post, PostError,
            UpdatePostContentDto, UpdatePostDto, UpdatePostTitleDto,
        },
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
    async fn create_post(&self, dto: CreatePostDto) -> Result<Post, Error> {
        let post = Post::new(dto)?;
        let created_post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, user_id, title, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO NOTHING
            RETURNING id, user_id, title, content, created_at, updated_at
            "#,
            post.id,
            post.user_id,
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

    async fn find_by_id(&self, dto: FindPostByIdDto) -> Result<Option<Post>, Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id, user_id, title, content, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
            dto.id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(post)
    }

    async fn find_by_user_id(&self, dto: FindPostByUserIdDto) -> Result<Vec<Post>, Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, user_id, title, content, created_at, updated_at
            FROM posts
            WHERE user_id = $1
            "#,
            dto.user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?;

        Ok(posts)
    }

    async fn update_post(&self, dto: UpdatePostDto) -> Result<Post, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1) as "exists!""#,
            dto.id
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
            RETURNING id, user_id, title, content, created_at, updated_at
            "#,
            dto.id,
            dto.new_title,
            dto.new_content,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Post(PostError::AlreadyExists))?;

        Ok(updated_post)
    }

    async fn update_title(&self, dto: UpdatePostTitleDto) -> Result<Post, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1) as "exists!""#,
            dto.id
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
            RETURNING id, user_id, title, content, created_at, updated_at
            "#,
            dto.id,
            dto.new_title,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Post(PostError::AlreadyExists))?;

        Ok(updated_post)
    }

    async fn update_content(&self, dto: UpdatePostContentDto) -> Result<Post, Error> {
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1) as "exists!""#,
            dto.id
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
            RETURNING id, user_id, title, content, created_at, updated_at
            "#,
            dto.id,
            dto.new_content,
            chrono::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::System(SystemError::Database(e.to_string())))?
        .ok_or(Error::Post(PostError::NotFound))?;

        Ok(updated_content)
    }

    async fn delete_post(&self, dto: DeletePostDto) -> Result<(), Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
            dto.id
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn create_test_user(pool: &PgPool) -> Uuid {
        let user_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(user_id)
        .bind(format!("test_{}@example.com", user_id))
        .bind("hashed_password")
        .execute(pool)
        .await
        .expect("Failed to create test user");

        user_id
    }

    #[sqlx::test]
    async fn test_create_post(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresPostRepository::new(pool);

        let dto = CreatePostDto {
            user_id,
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
        };

        let post = repo.create_post(dto).await.expect("Failed to create post");

        assert_eq!(post.user_id, user_id);
        assert_eq!(post.title, "Test Title");
        assert_eq!(post.content, "Test Content");
        assert!(post.id != Uuid::nil());
        assert!(post.created_at <= Utc::now());
        Ok(())
    }

    #[sqlx::test]
    async fn test_find_by_id(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresPostRepository::new(pool);

        let dto = CreatePostDto {
            user_id,
            title: "Find Me".to_string(),
            content: "Content".to_string(),
        };
        let created = repo.create_post(dto).await.expect("Failed to create");

        let found = repo
            .find_by_id(FindPostByIdDto { id: created.id })
            .await
            .expect("Failed to find");

        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_by_id_not_found(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresPostRepository::new(pool);

        let not_found = repo
            .find_by_id(FindPostByIdDto { id: Uuid::new_v4() })
            .await
            .expect("Failed to search");
        assert!(not_found.is_none());

        Ok(())
    }

    #[sqlx::test]
    async fn test_find_by_user_id(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let other_user_id = create_test_user(&pool).await;
        let repo = PostgresPostRepository::new(pool);

        repo.create_post(CreatePostDto {
            user_id,
            title: "Post 1".into(),
            content: "C1".into(),
        })
        .await
        .unwrap();

        repo.create_post(CreatePostDto {
            user_id,
            title: "Post 2".into(),
            content: "C2".into(),
        })
        .await
        .unwrap();

        repo.create_post(CreatePostDto {
            user_id: other_user_id,
            title: "Other Post".into(),
            content: "C3".into(),
        })
        .await
        .unwrap();

        let posts = repo
            .find_by_user_id(FindPostByUserIdDto { user_id })
            .await
            .expect("Failed to find posts");

        assert_eq!(posts.len(), 2);
        assert!(posts.iter().all(|p| p.user_id == user_id));
        Ok(())
    }

    #[sqlx::test]
    async fn test_update_post(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresPostRepository::new(pool);

        let created = repo
            .create_post(CreatePostDto {
                user_id,
                title: "Old Title".into(),
                content: "Old Content".into(),
            })
            .await
            .unwrap();

        let updated = repo
            .update_post(UpdatePostDto {
                id: created.id,
                new_title: "New Title".into(),
                new_content: "New Content".into(),
            })
            .await
            .expect("Failed to update");

        assert_eq!(updated.title, "New Title");
        assert_eq!(updated.content, "New Content");
        assert!(updated.updated_at > created.updated_at);

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_title(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresPostRepository::new(pool);

        let created = repo
            .create_post(CreatePostDto {
                user_id,
                title: "Old Title".into(),
                content: "Content".into(),
            })
            .await
            .unwrap();

        let updated = repo
            .update_title(UpdatePostTitleDto {
                id: created.id,
                new_title: "Only Title Changed".into(),
            })
            .await
            .expect("Failed to update title");

        assert_eq!(updated.title, "Only Title Changed");
        assert_eq!(updated.content, "Content");

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_content(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresPostRepository::new(pool);

        let created = repo
            .create_post(CreatePostDto {
                user_id,
                title: "Title".into(),
                content: "Old Content".into(),
            })
            .await
            .unwrap();

        let updated = repo
            .update_content(UpdatePostContentDto {
                id: created.id,
                new_content: "New Content".into(),
            })
            .await
            .expect("Failed to update content");

        assert_eq!(updated.title, "Title");
        assert_eq!(updated.content, "New Content");

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_not_found(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresPostRepository::new(pool);

        let result = repo
            .update_post(UpdatePostDto {
                id: Uuid::new_v4(),
                new_title: "Title".into(),
                new_content: "Content".into(),
            })
            .await;

        assert!(matches!(result, Err(Error::Post(PostError::NotFound))));

        Ok(())
    }

    #[sqlx::test]
    async fn test_delete_post(pool: PgPool) -> sqlx::Result<()> {
        let user_id = create_test_user(&pool).await;
        let repo = PostgresPostRepository::new(pool);

        let created = repo
            .create_post(CreatePostDto {
                user_id,
                title: "To Delete".into(),
                content: "Content".into(),
            })
            .await
            .unwrap();

        repo.delete_post(DeletePostDto { id: created.id })
            .await
            .expect("Failed to delete");

        let found = repo
            .find_by_id(FindPostByIdDto { id: created.id })
            .await
            .unwrap();
        assert!(found.is_none());

        let result = repo.delete_post(DeletePostDto { id: created.id }).await;
        assert!(matches!(result, Err(Error::Post(PostError::NotFound))));

        Ok(())
    }
}
