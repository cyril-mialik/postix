use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{errors::Error, post::PostError},
    repositories::{bookmark::BookmarkRepository, post::PostRepository},
};

#[derive(Clone)]
pub struct BookmarkService {
    bookmark_repo: Arc<dyn BookmarkRepository>,
    post_repo: Arc<dyn PostRepository>,
}

impl BookmarkService {
    pub fn new(
        bookmark_repo: Arc<dyn BookmarkRepository>,
        post_repo: Arc<dyn PostRepository>,
    ) -> Self {
        Self {
            bookmark_repo,
            post_repo,
        }
    }

    pub async fn get_bookmarks(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error> {
        self.bookmark_repo
            .find_by_profile(user_id, limit, offset)
            .await
    }

    pub async fn bookmark(&self, user_id: Uuid, post_id: Uuid) -> Result<(), Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;
        post.ensure_visible()?;

        self.bookmark_repo.create(user_id, post_id).await
    }

    pub async fn unbookmark(&self, user_id: Uuid, post_id: Uuid) -> Result<(), Error> {
        self.bookmark_repo.delete(user_id, post_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        entities::{
            post::{CreatePostDtoBuilder, Post},
            post_status::PostStatus,
        },
        repositories::{bookmark::MockBookmarkRepository, post::MockPostRepository},
    };

    fn published_post() -> Post {
        let mut post = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(Uuid::new_v4())
                .title("A public story".to_owned())
                .content(serde_json::json!({"text": "Story"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        post.status = PostStatus::Published;
        post
    }

    #[actix_web::test]
    async fn bookmarking_a_public_post_uses_the_authenticated_user_id() {
        let user_id = Uuid::new_v4();
        let post = published_post();
        let post_id = post.id;

        let mut posts = MockPostRepository::new();
        posts
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post.clone())));

        let mut bookmarks = MockBookmarkRepository::new();
        bookmarks
            .expect_create()
            .withf(move |actual_user_id, actual_post_id| {
                *actual_user_id == user_id && *actual_post_id == post_id
            })
            .once()
            .returning(|_, _| Ok(()));

        BookmarkService::new(Arc::new(bookmarks), Arc::new(posts))
            .bookmark(user_id, post_id)
            .await
            .unwrap();
    }

    #[actix_web::test]
    async fn bookmarking_a_draft_is_rejected() {
        let mut post = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(Uuid::new_v4())
                .title("A private draft".to_owned())
                .content(serde_json::json!({"text": "Draft"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        post.status = PostStatus::Draft;
        let post_id = post.id;

        let mut posts = MockPostRepository::new();
        posts
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post.clone())));
        let bookmarks = MockBookmarkRepository::new();

        let result = BookmarkService::new(Arc::new(bookmarks), Arc::new(posts))
            .bookmark(Uuid::new_v4(), post_id)
            .await;

        assert!(matches!(result, Err(Error::Post(PostError::NotFound))));
    }
}
