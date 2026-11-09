use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Draft,
    Published,
    Archived,
    Deleted,
}

impl PostStatus {
    const DRAFT: &str = "draft";
    const PUBLISHED: &str = "published";
    const ARCHIVED: &str = "archived";
    const DELETED: &str = "deleted";

    const DRAFT_IDX: i32 = 1;
    const PUBLISHED_IDX: i32 = 2;
    const ARCHIVED_IDX: i32 = 3;
    const DELETED_IDX: i32 = 4;

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            Self::DRAFT_IDX => Some(PostStatus::Draft),
            Self::PUBLISHED_IDX => Some(PostStatus::Published),
            Self::ARCHIVED_IDX => Some(PostStatus::Archived),
            Self::DELETED_IDX => Some(PostStatus::Deleted),
            _ => None,
        }
    }

    pub fn as_id(&self) -> i32 {
        match self {
            PostStatus::Draft => Self::DRAFT_IDX,
            PostStatus::Published => Self::PUBLISHED_IDX,
            PostStatus::Archived => Self::ARCHIVED_IDX,
            PostStatus::Deleted => Self::DELETED_IDX,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PostStatus::Draft => Self::DRAFT,
            PostStatus::Published => Self::PUBLISHED,
            PostStatus::Archived => Self::ARCHIVED,
            PostStatus::Deleted => Self::DELETED,
        }
    }

    pub fn is_draft(&self) -> bool {
        matches!(self, PostStatus::Draft)
    }

    pub fn is_published(&self) -> bool {
        matches!(self, PostStatus::Published)
    }

    pub fn is_archived(&self) -> bool {
        matches!(self, PostStatus::Archived)
    }

    pub fn is_deleted(&self) -> bool {
        matches!(self, PostStatus::Deleted)
    }
}
