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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::post::PostRepository;
    use chrono::Utc;
    use mockall::mock;
    use mockall::predicate::*;
    use uuid::Uuid;

    mock! {
        pub PostRepo {}

        #[async_trait::async_trait]
        impl PostRepository for PostRepo {
            async fn create_post(&self, dto: CreatePostDto) -> Result<Post, Error>;
            async fn find_by_id(&self, dto: FindPostByIdDto) -> Result<Option<Post>, Error>;
            async fn find_by_user_id(&self, dto: FindPostByUserIdDto) -> Result<Vec<Post>, Error>;
            async fn update_post(&self, dto: UpdatePostDto) -> Result<Post, Error>;
            async fn update_title(&self, dto: UpdatePostTitleDto) -> Result<Post, Error>;
            async fn update_content(&self, dto: UpdatePostContentDto) -> Result<Post, Error>;
            async fn delete_post(&self, dto: DeletePostDto) -> Result<(), Error>;
        }
    }

    fn mock_post() -> Post {
        Post {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            title: "Valid Title".to_string(),
            content: "Valid content for the post".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn mock_create_dto(post: &Post) -> CreatePostDto {
        CreatePostDto {
            user_id: post.user_id,
            title: post.title.clone(),
            content: post.content.clone(),
        }
    }

    #[tokio::test]
    async fn test_create_post_success() {
        let mut mock_repo = MockPostRepo::new();
        let post = mock_post();
        let post_clone = post.clone();

        mock_repo.expect_find_by_id().once().returning(|_| Ok(None));

        mock_repo
            .expect_create_post()
            .withf(move |dto| dto.title == "Valid Title")
            .once()
            .returning(move |_| Ok(post_clone.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let result = service.create_post(mock_create_dto(&post)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Valid Title");
    }

    #[tokio::test]
    async fn test_create_post_already_exists() {
        let mut mock_repo = MockPostRepo::new();
        let post = mock_post();
        let post_clone = post.clone();

        mock_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post_clone.clone())));

        mock_repo.expect_create_post().never();

        let service = PostService::new(Arc::new(mock_repo));
        let result = service.create_post(mock_create_dto(&post)).await;

        assert!(matches!(result, Err(Error::Post(PostError::AlreadyExists))));
    }

    #[tokio::test]
    async fn test_create_post_invalid_title() {
        let mut mock_repo = MockPostRepo::new();
        mock_repo.expect_find_by_id().never();
        mock_repo.expect_create_post().never();

        let service = PostService::new(Arc::new(mock_repo));
        let dto = CreatePostDto {
            user_id: Uuid::new_v4(),
            title: "ab".to_string(),
            content: "Valid content".to_string(),
        };
        let result = service.create_post(dto).await;

        assert!(matches!(
            result,
            Err(Error::Post(PostError::InvalidTitleLength))
        ));
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let mut mock_repo = MockPostRepo::new();
        let id = Uuid::new_v4();
        let post = mock_post();
        let post_clone = post.clone();

        mock_repo
            .expect_find_by_id()
            .withf(move |dto| dto.id == id)
            .once()
            .returning(move |_| Ok(Some(post_clone.clone())));

        let service = PostService::new(Arc::new(mock_repo));
        let result = service.find_by_id(FindPostByIdDto { id }).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_find_by_user_id() {
        let mut mock_repo = MockPostRepo::new();
        let user_id = Uuid::new_v4();
        let posts = vec![mock_post(), mock_post()];
        let posts_clone = posts.clone();

        mock_repo
            .expect_find_by_user_id()
            .withf(move |dto| dto.user_id == user_id)
            .once()
            .returning(move |_| Ok(posts_clone.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let result = service
            .find_by_user_id(FindPostByUserIdDto { user_id })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_update_post_success() {
        let mut mock_repo = MockPostRepo::new();
        let id = Uuid::new_v4();
        let post = mock_post();
        let post_clone = post.clone();

        mock_repo
            .expect_update_post()
            .withf(move |dto| dto.id == id)
            .once()
            .returning(move |_| Ok(post_clone.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let result = service
            .update_post(UpdatePostDto {
                id,
                new_title: "New Title".to_string(),
                new_content: "New Content".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_title_success() {
        let mut mock_repo = MockPostRepo::new();
        let id = Uuid::new_v4();
        let post = mock_post();
        let post_for_find = post.clone();
        let post_for_update = post.clone();

        mock_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post_for_find.clone())));

        mock_repo
            .expect_update_title()
            .withf(move |dto| dto.id == id && dto.new_title == "New Title")
            .once()
            .returning(move |_| Ok(post_for_update.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let result = service
            .update_title(UpdatePostTitleDto {
                id,
                new_title: "  New Title  ".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_title_not_found() {
        let mut mock_repo = MockPostRepo::new();
        let id = Uuid::new_v4();

        mock_repo.expect_find_by_id().once().returning(|_| Ok(None));

        mock_repo.expect_update_title().never();

        let service = PostService::new(Arc::new(mock_repo));
        let result = service
            .update_title(UpdatePostTitleDto {
                id,
                new_title: "New Title".to_string(),
            })
            .await;

        assert!(matches!(result, Err(Error::Post(PostError::NotFound))));
    }

    #[tokio::test]
    async fn test_update_content_success() {
        let mut mock_repo = MockPostRepo::new();
        let id = Uuid::new_v4();
        let post = mock_post();
        let post_for_find = post.clone();
        let post_for_update = post.clone();

        mock_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post_for_find.clone())));

        mock_repo
            .expect_update_content()
            .withf(move |dto| dto.id == id && dto.new_content == "New Content")
            .once()
            .returning(move |_| Ok(post_for_update.clone()));

        let service = PostService::new(Arc::new(mock_repo));
        let result = service
            .update_content(UpdatePostContentDto {
                id,
                new_content: "  New Content  ".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_content_not_found() {
        let mut mock_repo = MockPostRepo::new();
        let id = Uuid::new_v4();

        mock_repo.expect_find_by_id().once().returning(|_| Ok(None));

        mock_repo.expect_update_content().never();

        let service = PostService::new(Arc::new(mock_repo));
        let result = service
            .update_content(UpdatePostContentDto {
                id,
                new_content: "New Content".to_string(),
            })
            .await;

        assert!(matches!(result, Err(Error::Post(PostError::NotFound))));
    }

    #[tokio::test]
    async fn test_delete_post_success() {
        let mut mock_repo = MockPostRepo::new();
        let id = Uuid::new_v4();

        mock_repo
            .expect_delete_post()
            .withf(move |dto| dto.id == id)
            .once()
            .returning(|_| Ok(()));

        let service = PostService::new(Arc::new(mock_repo));
        let result = service.delete_post(DeletePostDto { id }).await;

        assert!(result.is_ok());
    }
}
