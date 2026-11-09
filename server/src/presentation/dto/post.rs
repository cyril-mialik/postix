use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::post::Post;

#[derive(Debug, Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub content: serde_json::Value,
    #[serde(default)]
    pub tag_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMediaResponse {
    pub id: Uuid,
    pub url: String,
    pub media_type: String,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
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

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            author_id: post.author_id,
            title: post.title,
            content: post.content,
            status: post.status.as_str().to_owned(),
            rating_sum: post.rating_sum,
            rating_count: post.rating_count,
            reviews_count: post.reviews_count,
            views_count: post.views_count,
            media: post
                .media
                .into_iter()
                .map(|m| PostMediaResponse {
                    id: m.id,
                    url: m.url,
                    media_type: m.media_type.as_str().to_string(),
                    position: m.position,
                })
                .collect(),
            tags: post
                .tags
                .into_iter()
                .map(|t| TagResponse {
                    id: t.id,
                    name: t.name,
                })
                .collect(),
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsResponse {
    pub posts: Vec<PostResponse>,
}

impl From<Vec<Post>> for PostsResponse {
    fn from(posts: Vec<Post>) -> Self {
        Self {
            posts: posts.into_iter().map(PostResponse::from).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FeedQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Deserialize)]
pub struct AuthorPostsQuery {
    pub author_id: Uuid,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}
