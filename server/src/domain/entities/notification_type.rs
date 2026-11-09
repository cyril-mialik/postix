use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    Review,
    Rating,
    Follow,
    Bookmark,
    System,
}

impl NotificationType {
    const REVIEW_IDX: i32 = 1;
    const RATING_IDX: i32 = 2;
    const FOLLOW_IDX: i32 = 3;
    const BOOKMARK_IDX: i32 = 4;
    const SYSTEM_IDX: i32 = 5;

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            Self::REVIEW_IDX => Some(NotificationType::Review),
            Self::RATING_IDX => Some(NotificationType::Rating),
            Self::FOLLOW_IDX => Some(NotificationType::Follow),
            Self::BOOKMARK_IDX => Some(NotificationType::Bookmark),
            Self::SYSTEM_IDX => Some(NotificationType::System),
            _ => None,
        }
    }

    pub fn as_id(&self) -> i32 {
        match self {
            NotificationType::Review => Self::REVIEW_IDX,
            NotificationType::Rating => Self::RATING_IDX,
            NotificationType::Follow => Self::FOLLOW_IDX,
            NotificationType::Bookmark => Self::BOOKMARK_IDX,
            NotificationType::System => Self::SYSTEM_IDX,
        }
    }

    pub fn is_review(&self) -> bool {
        matches!(self, NotificationType::Review)
    }

    pub fn is_rating(&self) -> bool {
        matches!(self, NotificationType::Rating)
    }

    pub fn is_follow(&self) -> bool {
        matches!(self, NotificationType::Follow)
    }

    pub fn is_bookmark(&self) -> bool {
        matches!(self, NotificationType::Bookmark)
    }

    pub fn is_system(&self) -> bool {
        matches!(self, NotificationType::System)
    }
}
