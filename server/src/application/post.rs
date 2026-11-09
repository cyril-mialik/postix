use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        post::{CreatePostDtoBuilder, Post, PostError},
        user::UserError,
    },
    repositories::{post::PostRepository, tag::TagRepository, user::UserRepository},
};

#[derive(Clone)]
pub struct PostService {
    post_repo: Arc<dyn PostRepository>,
    tag_repo: Arc<dyn TagRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl PostService {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        tag_repo: Arc<dyn TagRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            post_repo,
            tag_repo,
            user_repo,
        }
    }

    pub async fn create_post(
        &self,
        author_id: Uuid,
        title: String,
        content: serde_json::Value,
        tag_names: Vec<String>,
    ) -> Result<Post, Error> {
        let mut tags = Vec::with_capacity(tag_names.len());
        for name in &tag_names {
            tags.push(self.tag_repo.find_or_create(name).await?);
        }

        let dto = CreatePostDtoBuilder::default()
            .author_id(author_id)
            .title(title)
            .content(content)
            .tags(tags)
            .build()
            .map_err(|e| Error::Post(PostError::Validation(e.to_string())))?;

        let post = Post::new(dto)?;
        let created = self.post_repo.create(&post).await?;

        self.user_repo.increment_posts_count(author_id, 1).await?;

        Ok(created)
    }

    pub async fn get_feed(&self, limit: i64, offset: i64) -> Result<Vec<Post>, Error> {
        self.post_repo.find_feed(limit, offset).await
    }

    pub async fn get_post(&self, post_id: Uuid) -> Result<Post, Error> {
        self.post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))
    }

    pub async fn get_posts_by_author(
        &self,
        author_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, Error> {
        self.post_repo
            .find_by_author(author_id, limit, offset)
            .await
    }

    pub async fn update_post(
        &self,
        user_id: Uuid,
        post_id: Uuid,
        title: Option<String>,
        content: Option<serde_json::Value>,
    ) -> Result<Post, Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;

        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;

        post.ensure_author_or_moderator(&user)?;

        let mut updated = post.clone();
        if let Some(title) = title {
            updated.title = title.trim().to_owned();
        }
        if let Some(content) = content {
            updated.content = content;
        }
        updated.updated_at = chrono::Utc::now();

        self.post_repo.update(&updated).await
    }

    pub async fn delete_post(&self, user_id: Uuid, post_id: Uuid) -> Result<(), Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;

        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;

        post.ensure_author_or_moderator(&user)?;

        self.post_repo.soft_delete(post_id, user_id).await?;
        self.user_repo
            .increment_posts_count(post.author_id, -1)
            .await?;

        Ok(())
    }
}
