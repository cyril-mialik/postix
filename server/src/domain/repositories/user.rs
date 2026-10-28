use crate::domain::entities::{errors::Error, user::User};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User, password_hash: &str) -> Result<User, Error>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;

    async fn find_credentials_by_email(&self, email: &str)
    -> Result<Option<(User, String)>, Error>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;

    async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User, Error>;

    async fn update_password(&self, id: Uuid, new_password: &str) -> Result<User, Error>;

    async fn delete_user(&self, id: Uuid) -> Result<(), Error>;
}
