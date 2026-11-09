use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

const REVIEW_MAX_LENGTH: usize = 5000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: i32,
    pub post_id: Uuid,
    pub profile_id: Uuid,
    pub review: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Review {
    pub fn new(post_id: Uuid, profile_id: Uuid, review: String) -> Result<Self, ReviewError> {
        let review = review.trim().to_owned();

        if review.is_empty() || review.len() > REVIEW_MAX_LENGTH {
            return Err(ReviewError::InvalidLength);
        }

        Ok(Self {
            id: 0,
            post_id,
            profile_id,
            review,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Ревью должно быть от 1 до 5000 символов")]
    InvalidLength,

    #[error("Ревью не найдено")]
    NotFound,

    #[error("Вы уже оставили ревью этому посту")]
    AlreadyExists,
}
