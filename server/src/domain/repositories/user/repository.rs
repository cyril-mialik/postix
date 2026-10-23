use crate::domain::entities::{errors::DomainError, user::entity::User};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &User, password_hash: &str) -> Result<User, DomainError>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;

    async fn find_credentials_by_email(
        &self,
        email: &str,
    ) -> Result<Option<(User, String)>, DomainError>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;

    async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User, DomainError>;

    async fn update_password(&self, id: Uuid, new_password: &str) -> Result<User, DomainError>;

    async fn delete_user(&self, id: Uuid) -> Result<(), DomainError>;
}
