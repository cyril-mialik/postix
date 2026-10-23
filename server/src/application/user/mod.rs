use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{
        errors::{DomainError, SystemError},
        user::{entity::User, errors::UserError},
    },
    repositories::user::repository::UserRepository,
};
use crate::infrastructure::security::password::hash_password;

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, email: String, password: &str) -> Result<User, DomainError> {
        let user = User::new(email)?;

        if self.repository.find_by_email(&user.email).await?.is_some() {
            return Err(UserError::EmailAlreadyExists.into());
        }

        let password_hash = hash_password(password)
            .map_err(|_| DomainError::System(SystemError::PasswordHashFailed))?;

        let created_user = self.repository.create_user(&user, &password_hash).await?;

        Ok(created_user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        self.repository.find_by_email(email).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        self.repository.find_by_id(id).await
    }

    pub async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User, DomainError> {
        let normalized = new_email.trim().to_lowercase();

        if normalized.is_empty() || !normalized.contains('@') {
            return Err(UserError::InvalidEmail.into());
        }

        self.repository.update_email(id, &normalized).await
    }

    pub async fn update_password(&self, id: Uuid, new_password: &str) -> Result<User, DomainError> {
        let password_hash = hash_password(new_password)
            .map_err(|_| DomainError::System(SystemError::PasswordHashFailed))?;

        self.repository.update_password(id, &password_hash).await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), DomainError> {
        self.repository.delete_user(id).await
    }
}
