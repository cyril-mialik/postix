use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // It used actually
pub struct PostQuery {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // It used actually
pub struct UpdatePostRequest {
    pub id: Uuid,
    pub new_title: String,
    pub new_content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // It used actually
pub struct UpdatePostTitleRequest {
    pub id: Uuid,
    pub new_title: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // It used actually
pub struct UpdatePostContentRequest {
    pub id: Uuid,
    pub new_content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // It used actually
pub struct DeletePostRequest {
    pub id: Uuid,
}

impl From<crate::domain::entities::post::Post> for PostResponse {
    fn from(post: crate::domain::entities::post::Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            created_at: post.created_at,
        }
    }
}
