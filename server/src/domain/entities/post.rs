use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Post {
    pub id: Uuid,

    #[validate(length(max = 256))]
    pub title: String,
    pub content: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub fn new(title: String, content: String) -> Result<Self, PostError> {
        let title = title.trim().to_owned();

        let new_post = Self {
            id: Uuid::new_v4(),
            title,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        new_post.validate().map_err(|_| PostError::InvalidTitleLength)?;

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
