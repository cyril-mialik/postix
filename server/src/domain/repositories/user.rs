use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{errors::Error, user::User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error>;
    async fn find_credentials_by_email(&self, email: &str)
    -> Result<Option<(User, String)>, Error>;
    async fn create(&self, user: &User) -> Result<User, Error>;
    async fn update(&self, user: &User) -> Result<User, Error>;
    async fn soft_delete(&self, id: Uuid) -> Result<(), Error>;
    async fn ban(&self, id: Uuid, reason: &str) -> Result<(), Error>;
    async fn unban(&self, id: Uuid) -> Result<(), Error>;
    async fn increment_posts_count(&self, id: Uuid, delta: i32) -> Result<(), Error>;
    async fn increment_followers_count(&self, id: Uuid, delta: i32) -> Result<(), Error>;
    async fn increment_following_count(&self, id: Uuid, delta: i32) -> Result<(), Error>;
}
