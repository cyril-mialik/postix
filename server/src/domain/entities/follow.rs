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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_follow_between_distinct_users() {
        let follower_id = Uuid::from_u128(1);
        let following_id = Uuid::from_u128(2);

        let follow = Follow::new(follower_id, following_id).unwrap();

        assert_eq!(follow.follower_id, follower_id);
        assert_eq!(follow.following_id, following_id);
    }

    #[test]
    fn rejects_following_yourself() {
        let user_id = Uuid::from_u128(1);

        assert!(matches!(
            Follow::new(user_id, user_id),
            Err(FollowError::CannotFollowSelf)
        ));
    }
}
