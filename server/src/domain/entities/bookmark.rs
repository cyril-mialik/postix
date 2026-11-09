use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub profile_id: Uuid,
    pub post_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Bookmark {
    pub fn new(profile_id: Uuid, post_id: Uuid) -> Self {
        Self {
            profile_id,
            post_id,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Error)]
pub enum BookmarkError {
    #[error("Закладка уже существует")]
    AlreadyExists,

    #[error("Закладка не найдена")]
    NotFound,
}
