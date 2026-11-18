use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{bookmark::Bookmark, errors::Error, post::PostError},
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
        profile_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Uuid>, Error> {
        self.bookmark_repo
            .find_by_profile(profile_id, limit, offset)
            .await
    }

    pub async fn bookmark(&self, profile_id: Uuid, post_id: Uuid) -> Result<(), Error> {
        self.post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;

        let bookmark = Bookmark::new(profile_id, post_id);
        self.bookmark_repo.create(&bookmark).await
    }

    pub async fn unbookmark(&self, profile_id: Uuid, post_id: Uuid) -> Result<(), Error> {
        self.bookmark_repo.delete(profile_id, post_id).await
    }
}
