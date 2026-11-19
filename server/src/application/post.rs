use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::Error,
        post::{CreatePostDtoBuilder, Post, PostError},
        post_status::PostStatus,
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
        status: PostStatus,
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

        let mut post = Post::new(dto)?;
        post.status = status;
        let created = self.post_repo.create(&post).await?;

        Ok(created)
    }

    pub async fn get_feed(&self, limit: i64, offset: i64) -> Result<Vec<Post>, Error> {
        let posts = self.post_repo.find_feed(limit, offset).await?;
        Ok(posts.into_iter().filter(Post::is_visible).collect())
    }

    pub async fn get_post(&self, post_id: Uuid) -> Result<Post, Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;
        post.ensure_visible().map_err(Error::Post)?;
        Ok(post)
    }

    pub async fn get_post_for_user(&self, user_id: Uuid, post_id: Uuid) -> Result<Post, Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;

        if post.is_visible() {
            return Ok(post);
        }

        if post.status.is_draft() && post.deleted_at.is_none() {
            let user = self
                .user_repo
                .find_by_id(user_id)
                .await?
                .ok_or(Error::User(UserError::NotFound))?;
            post.ensure_author_or_moderator(&user)?;
            return Ok(post);
        }

        Err(Error::Post(PostError::NotFound))
    }

    pub async fn get_posts_by_author(
        &self,
        author_id: Uuid,
        status: PostStatus,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, Error> {
        let status_id = status.as_id();
        let posts = self
            .post_repo
            .find_by_author(author_id, status_id, limit, offset)
            .await?;
        Ok(posts
            .into_iter()
            .filter(|post| post.deleted_at.is_none())
            .collect())
    }

    pub async fn update_post(
        &self,
        user_id: Uuid,
        post_id: Uuid,
        title: Option<String>,
        content: Option<serde_json::Value>,
        tag_names: Option<Vec<String>>,
    ) -> Result<Post, Error> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or(Error::Post(PostError::NotFound))?;

        if post.deleted_at.is_some() || post.status.is_deleted() {
            return Err(Error::Post(PostError::NotFound));
        }

        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(Error::User(UserError::NotFound))?;

        post.ensure_author_or_moderator(&user)?;

        let mut updated = post.clone();
        if let Some(title) = title {
            let title = title.trim();
            if !(3..=256).contains(&title.chars().count()) {
                return Err(Error::Post(PostError::InvalidTitleLength));
            }
            updated.title = title.to_owned();
        }
        if let Some(content) = content {
            updated.content = content;
        }
        if let Some(tag_names) = tag_names {
            let mut tags = Vec::new();
            for name in tag_names {
                let name = name.trim().to_lowercase();
                if name.is_empty()
                    || tags
                        .iter()
                        .any(|tag: &crate::domain::entities::tag::Tag| tag.name == name)
                {
                    continue;
                }
                if name.chars().count() > 50 {
                    return Err(Error::Post(PostError::Validation(
                        "Tags must be 50 characters or fewer.".to_owned(),
                    )));
                }
                tags.push(self.tag_repo.find_or_create(&name).await?);
            }
            updated.tags = tags;
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

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::domain::{
        entities::{
            post::{CreatePostDtoBuilder, Post},
            role::Role,
            theme::Theme,
            user::User,
        },
        repositories::{
            post::MockPostRepository, tag::MockTagRepository, user::MockUserRepository,
        },
    };

    fn draft(author_id: Uuid) -> Post {
        let mut post = Post::new(
            CreatePostDtoBuilder::default()
                .author_id(author_id)
                .title("A private draft".to_owned())
                .content(serde_json::json!({"text": "Draft content"}))
                .build()
                .unwrap(),
        )
        .unwrap();
        post.status = PostStatus::Draft;
        post
    }

    fn user(id: Uuid) -> User {
        User {
            id,
            email: "author@example.com".to_owned(),
            password_hash: "hash".to_owned(),
            is_email_verified: false,
            is_banned: false,
            ban_reason: None,
            banned_at: None,
            role: Role::User,
            username: "author".to_owned(),
            avatar: None,
            first_name: None,
            last_name: None,
            followers_count: 0,
            following_count: 0,
            posts_count: 0,
            theme: Theme::Auto,
            deleted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn service(post: Post, reader: User) -> PostService {
        let mut post_repo = MockPostRepository::new();
        post_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(post.clone())));

        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(reader.clone())));

        PostService::new(
            Arc::new(post_repo),
            Arc::new(MockTagRepository::new()),
            Arc::new(user_repo),
        )
    }

    #[actix_web::test]
    async fn draft_posts_are_visible_to_their_author() {
        let author_id = Uuid::new_v4();
        let post = draft(author_id);

        let result = service(post.clone(), user(author_id))
            .get_post_for_user(author_id, post.id)
            .await
            .unwrap();

        assert_eq!(result.id, post.id);
        assert!(result.status.is_draft());
    }

    #[actix_web::test]
    async fn draft_posts_are_not_visible_to_other_users() {
        let author_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let post = draft(author_id);

        let result = service(post.clone(), user(other_user_id))
            .get_post_for_user(other_user_id, post.id)
            .await;

        assert!(matches!(result, Err(Error::Post(PostError::Forbidden))));
    }

    #[actix_web::test]
    async fn update_post_changes_title_content_and_tags_for_the_author() {
        let author_id = Uuid::new_v4();
        let post = draft(author_id);
        let post_id = post.id;

        let mut post_repo = MockPostRepository::new();
        post_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(post.clone())));
        post_repo
            .expect_update()
            .once()
            .returning(|post| Ok(post.clone()));

        let mut tag_repo = MockTagRepository::new();
        tag_repo.expect_find_or_create().once().returning(|name| {
            Ok(crate::domain::entities::tag::Tag {
                id: 8,
                name: name.to_owned(),
            })
        });

        let mut user_repo = MockUserRepository::new();
        let author = user(author_id);
        user_repo
            .expect_find_by_id()
            .once()
            .returning(move |_| Ok(Some(author.clone())));

        let updated =
            PostService::new(Arc::new(post_repo), Arc::new(tag_repo), Arc::new(user_repo))
                .update_post(
                    author_id,
                    post_id,
                    Some(" Updated title ".to_owned()),
                    Some(serde_json::json!({"text": "Updated body"})),
                    Some(vec!["Rust".to_owned(), " ".to_owned()]),
                )
                .await
                .unwrap();

        assert_eq!(updated.title, "Updated title");
        assert_eq!(updated.content, serde_json::json!({"text": "Updated body"}));
        assert_eq!(updated.tags.len(), 1);
        assert_eq!(updated.tags[0].name, "rust");
    }
}
