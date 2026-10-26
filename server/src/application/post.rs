use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    entities::{
        errors::Error,
        post::{Post, PostError},
    },
    repositories::post::PostRepository,
};

pub struct PostService {
    repository: Arc<dyn PostRepository>,
}

impl PostService {
    pub fn new(repository: Arc<dyn PostRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_post(&self, title: String, content: String) -> Result<Post, Error> {
        let post = Post::new(title, content)?;

        if self.repository.find_by_id(post.id).await?.is_some() {
            return Err(PostError::AlreadyExists.into());
        }

        let created_post = self.repository.create_post(&post).await?;

        Ok(created_post)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn update_post(&self, id: Uuid, new_title: &str, new_content: &str) -> Result<Post, Error> {
        self.repository.update_post(id, new_title, new_content).await
    }

    pub async fn update_title(&self, id: Uuid, new_title: &str) -> Result<Post, Error> {
        let normalized = new_title.trim();
        let post = self.find_by_id(id).await?.ok_or(PostError::NotFound)?;

        post.validate().map_err(|_| PostError::InvalidTitleLength)?;

        self.repository.update_title(id, &normalized).await
    }

    pub async fn update_content(&self, id: Uuid, new_content: &str) -> Result<Post, Error> {
        let normalized = new_content.trim();
        let _post = self.find_by_id(id).await?.ok_or(PostError::NotFound)?;

        self.repository.update_content(id, &normalized).await
    }

    pub async fn delete_post(&self, id: Uuid) -> Result<(), Error> {
        self.repository.delete_post(id).await
    }
}
