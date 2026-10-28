use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone)]
pub struct CreatePostDto {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

impl From<Post> for CreatePostDto {
    fn from(post: Post) -> Self {
        Self {
            user_id: post.user_id,
            title: post.title,
            content: post.content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdatePostDto {
    pub id: Uuid,
    pub new_title: String,
    pub new_content: String,
}

impl From<Post> for UpdatePostDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            new_title: post.title,
            new_content: post.content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FindPostByIdDto {
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub struct FindPostByUserIdDto {
    pub user_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct UpdatePostTitleDto {
    pub id: Uuid,
    pub new_title: String,
}

impl From<Post> for UpdatePostTitleDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            new_title: post.title,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdatePostContentDto {
    pub id: Uuid,
    pub new_content: String,
}

impl From<Post> for UpdatePostContentDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            new_content: post.content,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeletePostDto {
    pub id: Uuid,
}

impl From<Post> for DeletePostDto {
    fn from(post: Post) -> Self {
        Self { id: post.id }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,

    #[validate(length(max = 256))]
    pub title: String,
    pub content: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub fn new(dto: CreatePostDto) -> Result<Self, PostError> {
        let content = dto.content.trim().to_owned();
        let title = dto.title.trim().to_owned();
        let user_id = dto.user_id;

        let new_post = Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        new_post
            .validate()
            .map_err(|_| PostError::InvalidTitleLength)?;

        Ok(new_post)
    }
}

#[derive(Debug, Error)]
pub enum PostError {
    #[error("Пост с таким id уже существует")]
    AlreadyExists,

    #[error("Пользователь не найден")]
    NotFound,

    #[error("Неверная длина заголовка")]
    InvalidTitleLength,
}
