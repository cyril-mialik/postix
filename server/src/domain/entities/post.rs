use chrono::{DateTime, Utc};
use derive_builder::{Builder, UninitializedFieldError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

use crate::domain::entities::tag::Tag;

use super::{media_type::MediaType, post_status::PostStatus, user::User};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,

    #[validate(length(min = 3, max = 256))]
    pub title: String,
    pub content: serde_json::Value,
    pub status: PostStatus,

    pub rating_sum: i32,
    pub rating_count: i32,
    pub reviews_count: i32,
    pub views_count: i32,

    pub media: Vec<PostMedia>,
    pub tags: Vec<Tag>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

impl Post {
    pub const INCREMENT: i32 = 1;
    pub const DECREMENT: i32 = -1;

    pub fn new(dto: CreatePostDto) -> Result<Self, PostError> {
        let post = Self {
            id: Uuid::new_v4(),
            author_id: dto.author_id,
            title: dto.title.trim().to_owned(),
            content: dto.content,
            status: PostStatus::Draft,
            rating_sum: 0,
            rating_count: 0,
            reviews_count: 0,
            views_count: 0,
            media: dto.media,
            tags: dto.tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            deleted_by: None,
        };

        post.validate().map_err(|e| {
            let field_errors = e.field_errors();
            if field_errors.contains_key("title") {
                return PostError::InvalidTitleLength;
            }
            PostError::Validation(e.to_string())
        })?;

        Ok(post)
    }

    pub fn average_rating(&self) -> f64 {
        if self.rating_count == 0 {
            0.0
        } else {
            self.rating_sum as f64 / self.rating_count as f64
        }
    }

    pub fn is_visible(&self) -> bool {
        self.status == PostStatus::Published && self.deleted_at.is_none()
    }

    pub fn ensure_author_or_moderator(&self, user: &User) -> Result<(), PostError> {
        if self.author_id != user.id && !user.role.is_moderator_or_admin() {
            return Err(PostError::Forbidden);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMedia {
    pub id: Uuid,
    pub url: String,
    pub media_type: MediaType,
    pub position: i32,
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into), build_fn(error = "CreatePostDtoBuilderError"))]
pub struct CreatePostDto {
    pub author_id: Uuid,
    pub title: String,
    pub content: serde_json::Value,

    #[builder(default)]
    pub media: Vec<PostMedia>,

    #[builder(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Error)]
pub enum CreatePostDtoBuilderError {
    #[error(transparent)]
    UninitializedField(#[from] UninitializedFieldError),
}

#[derive(Debug, Error)]
pub enum PostError {
    #[error("Пост не найден")]
    NotFound,

    #[error("Пост уже существует")]
    AlreadyExists,

    #[error("Неверная длина заголовка (3–256)")]
    InvalidTitleLength,

    #[error("Доступ запрещён")]
    Forbidden,

    #[error("Ошибка валидации: {0}")]
    Validation(String),
}
