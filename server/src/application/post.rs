use std::sync::Arc;
use validator::Validate;

use crate::domain::{
    entities::{
        errors::Error,
        post::{
            CreatePostDto, DeletePostDto, FindPostByIdDto, FindPostByUserIdDto, Post, PostError,
            UpdatePostContentDto, UpdatePostDto, UpdatePostTitleDto,
        },
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

    pub async fn create_post(&self, dto: CreatePostDto) -> Result<Post, Error> {
        let post = Post::new(dto.clone())?;

        if self
            .repository
            .find_by_id(FindPostByIdDto { id: post.id })
            .await?
            .is_some()
        {
            return Err(PostError::AlreadyExists.into());
        }

        let created_post = self.repository.create_post(dto).await?;

        Ok(created_post)
    }

    pub async fn find_by_id(&self, dto: FindPostByIdDto) -> Result<Option<Post>, Error> {
        self.repository.find_by_id(dto).await
    }

    pub async fn find_by_user_id(&self, dto: FindPostByUserIdDto) -> Result<Vec<Post>, Error> {
        self.repository.find_by_user_id(dto).await
    }

    pub async fn update_post(&self, dto: UpdatePostDto) -> Result<Post, Error> {
        self.repository.update_post(dto).await
    }

    pub async fn update_title(&self, dto: UpdatePostTitleDto) -> Result<Post, Error> {
        let normalized = dto.new_title.trim().to_owned();
        let post = self
            .find_by_id(FindPostByIdDto { id: dto.id })
            .await?
            .ok_or(PostError::NotFound)?;

        post.validate().map_err(|_| PostError::InvalidTitleLength)?;

        self.repository
            .update_title(UpdatePostTitleDto {
                id: dto.id,
                new_title: normalized,
            })
            .await
    }

    pub async fn update_content(&self, dto: UpdatePostContentDto) -> Result<Post, Error> {
        let normalized = dto.new_content.trim().to_owned();
        let _post = self
            .find_by_id(FindPostByIdDto { id: dto.id })
            .await?
            .ok_or(PostError::NotFound)?;

        self.repository
            .update_content(UpdatePostContentDto {
                id: dto.id,
                new_content: normalized,
            })
            .await
    }

    pub async fn delete_post(&self, dto: DeletePostDto) -> Result<(), Error> {
        self.repository.delete_post(dto).await
    }
}
