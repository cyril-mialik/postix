use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub id: i32,
    pub post_id: Uuid,
    pub profile_id: Uuid,
    pub value: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Rating {
    const LOWER_RATING: i32 = 1;
    const UPPER_RATING: i32 = 5;

    pub fn new(post_id: Uuid, profile_id: Uuid, value: i32) -> Result<Self, RatingError> {
        Self::check_value(value)?;

        Ok(Self {
            id: 0,
            post_id,
            profile_id,
            value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub fn check_value(value: i32) -> Result<(), RatingError> {
        if value < Self::LOWER_RATING || value > Self::UPPER_RATING {
            return Err(RatingError::InvalidValue);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RatingError {
    #[error("Рейтинг должен быть от 1 до 5")]
    InvalidValue,

    #[error("Рейтинг не найден")]
    NotFound,

    #[error("Вы уже поставили рейтинг этому посту")]
    AlreadyExists,
}
