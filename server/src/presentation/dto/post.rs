use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::post::{
    CreatePostDto, DeletePostDto, FindPostByIdDto, FindPostByUserIdDto, Post, UpdatePostContentDto,
    UpdatePostDto, UpdatePostTitleDto,
};

#[derive(Debug, Deserialize)]
pub struct PostRequest {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

impl Into<CreatePostDto> for PostRequest {
    fn into(self) -> CreatePostDto {
        CreatePostDto {
            user_id: self.user_id,
            title: self.title,
            content: self.content,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            created_at: post.created_at,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
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
pub struct QueryPostId {
    pub id: Uuid,
}

impl Into<FindPostByIdDto> for QueryPostId {
    fn into(self) -> FindPostByIdDto {
        FindPostByIdDto { id: self.id }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryPostUserId {
    pub user_id: Uuid,
}

impl Into<FindPostByUserIdDto> for QueryPostUserId {
    fn into(self) -> FindPostByUserIdDto {
        FindPostByUserIdDto {
            user_id: self.user_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub id: Uuid,
    pub new_title: String,
    pub new_content: String,
}

impl Into<UpdatePostDto> for UpdatePostRequest {
    fn into(self) -> UpdatePostDto {
        UpdatePostDto {
            id: self.id,
            new_title: self.new_title,
            new_content: self.new_content,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostTitleRequest {
    pub id: Uuid,
    pub new_title: String,
}

impl Into<UpdatePostTitleDto> for UpdatePostTitleRequest {
    fn into(self) -> UpdatePostTitleDto {
        UpdatePostTitleDto {
            id: self.id,
            new_title: self.new_title,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostContentRequest {
    pub id: Uuid,
    pub new_content: String,
}

impl Into<UpdatePostContentDto> for UpdatePostContentRequest {
    fn into(self) -> UpdatePostContentDto {
        UpdatePostContentDto {
            id: self.id,
            new_content: self.new_content,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeletePostRequest {
    pub id: Uuid,
}

impl Into<DeletePostDto> for DeletePostRequest {
    fn into(self) -> DeletePostDto {
        DeletePostDto { id: self.id }
    }
}
