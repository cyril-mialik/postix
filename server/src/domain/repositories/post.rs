use crate::domain::entities::{
    errors::Error,
    post::{
        CreatePostDto, DeletePostDto, FindPostByIdDto, FindPostByUserIdDto, Post,
        UpdatePostContentDto, UpdatePostDto, UpdatePostTitleDto,
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, dto: CreatePostDto) -> Result<Post, Error>;

    async fn find_by_id(&self, dto: FindPostByIdDto) -> Result<Option<Post>, Error>;

    async fn find_by_user_id(&self, dto: FindPostByUserIdDto) -> Result<Vec<Post>, Error>;

    async fn update_title(&self, dto: UpdatePostTitleDto) -> Result<Post, Error>;

    async fn update_post(&self, dto: UpdatePostDto) -> Result<Post, Error>;

    async fn update_content(&self, dto: UpdatePostContentDto) -> Result<Post, Error>;

    async fn delete_post(&self, dto: DeletePostDto) -> Result<(), Error>;
}
