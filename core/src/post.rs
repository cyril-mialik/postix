use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PostRequest {
    pub title: String,
    pub content: serde_json::Value,
    #[serde(default)]
    pub tag_names: Vec<String>,
    #[serde(default)]
    pub status: PostPublicationStatus,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PostPublicationStatus {
    Draft,
    #[default]
    Published,
}

impl PostPublicationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<serde_json::Value>,
    pub tag_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostMediaResponse {
    pub id: Uuid,
    pub url: String,
    pub media_type: String,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    #[serde(default)]
    pub author_username: String,
    #[serde(default)]
    pub author_avatar: Option<String>,
    pub title: String,
    pub content: serde_json::Value,
    pub status: String,
    pub rating_sum: i32,
    pub rating_count: i32,
    pub reviews_count: i32,
    pub views_count: i32,
    pub media: Vec<PostMediaResponse>,
    pub tags: Vec<TagResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PostRatingRequest {
    pub rating: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PostRatingSummary {
    pub rating_sum: i32,
    pub rating_count: i32,
    pub my_rating: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<PostResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    #[serde(default)]
    pub status: Option<PostPublicationStatus>,
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorPostsQuery {
    pub author_id: Uuid,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}
