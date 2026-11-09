use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follow {
    pub follower_id: Uuid,
    pub following_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Follow {
    pub fn new(follower_id: Uuid, following_id: Uuid) -> Result<Self, FollowError> {
        if follower_id == following_id {
            return Err(FollowError::CannotFollowSelf);
        }

        Ok(Self {
            follower_id,
            following_id,
            created_at: Utc::now(),
        })
    }
}

#[derive(Debug, Error)]
pub enum FollowError {
    #[error("Нельзя подписаться на себя")]
    CannotFollowSelf,

    #[error("Вы уже подписаны")]
    AlreadyFollowing,

    #[error("Подписка не найдена")]
    NotFound,
}
