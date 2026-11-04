use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::domain::{
    entities::{
        errors::{Error, SystemError},
        user::{User, UserError},
    },
    repositories::user::UserRepository,
};
use crate::infrastructure::security::password::hash_password;

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, email: String, password: &str) -> Result<User, Error> {
        let user = User::new(email)?;

        if self.repository.find_by_email(&user.email).await?.is_some() {
            return Err(UserError::EmailAlreadyExists.into());
        }

        let password_hash =
            hash_password(password).map_err(|_| Error::System(SystemError::PasswordHashFailed))?;

        let created_user = self.repository.create_user(&user, &password_hash).await?;

        Ok(created_user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        self.repository.find_by_email(email).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User, Error> {
        let normalized = new_email.trim().to_lowercase();
        let user = self
            .find_by_email(&normalized)
            .await?
            .ok_or(UserError::UserNotFound)?;

        user.validate().map_err(|_| UserError::InvalidEmail)?;

        self.repository.update_email(id, &normalized).await
    }

    pub async fn update_password(&self, id: Uuid, new_password: &str) -> Result<User, Error> {
        let hashed_password = hash_password(new_password)
            .map_err(|_| Error::System(SystemError::PasswordHashFailed))?;

        self.repository.update_password(id, &hashed_password).await
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), Error> {
        self.repository.delete_user(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::user::UserRepository;
    use mockall::mock;
    use mockall::predicate::*;

    mock! {
        pub UserRepo {}

        #[async_trait::async_trait]
        impl UserRepository for UserRepo {
            async fn create_user(&self, user: &User, password_hash: &str) -> Result<User, Error>;
            async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
            async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User, Error>;
            async fn update_password(&self, id: Uuid, new_password: &str) -> Result<User, Error>;
            async fn delete_user(&self, id: Uuid) -> Result<(), Error>;
            async fn find_credentials_by_email(&self, email: &str) -> Result<Option<(User, String)>, Error>;
        }
    }

    fn mock_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let mut mock_repo = MockUserRepo::new();
        let user = mock_user();
        let user_clone = user.clone();

        mock_repo
            .expect_find_by_email()
            .with(eq("test@example.com"))
            .once()
            .returning(|_| Ok(None));

        mock_repo
            .expect_create_user()
            .withf(move |u, _hash| u.email == "test@example.com")
            .once()
            .returning(move |_, _| Ok(user_clone.clone()));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service
            .create_user("test@example.com".into(), "password123")
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_create_user_already_exists() {
        let mut mock_repo = MockUserRepo::new();
        let user = mock_user();

        mock_repo
            .expect_find_by_email()
            .with(eq("test@example.com"))
            .once()
            .returning(move |_| Ok(Some(user.clone())));

        mock_repo.expect_create_user().never();

        let service = UserService::new(Arc::new(mock_repo));
        let result = service
            .create_user("test@example.com".into(), "password123")
            .await;

        assert!(matches!(
            result,
            Err(Error::User(UserError::EmailAlreadyExists))
        ));
    }

    #[tokio::test]
    async fn test_create_user_invalid_email() {
        let mut mock_repo = MockUserRepo::new();
        mock_repo.expect_find_by_email().never();
        mock_repo.expect_create_user().never();

        let service = UserService::new(Arc::new(mock_repo));
        let result = service
            .create_user("invalid-email".into(), "password123")
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_email_success() {
        let mut mock_repo = MockUserRepo::new();
        let id = Uuid::new_v4();
        let user = mock_user();
        let user_for_find = user.clone();
        let user_for_update = user.clone();

        mock_repo
            .expect_find_by_email()
            .with(eq("new@example.com"))
            .once()
            .returning(move |_| Ok(Some(user_for_find.clone())));

        mock_repo
            .expect_update_email()
            .with(eq(id), eq("new@example.com"))
            .once()
            .returning(move |_, _| Ok(user_for_update.clone()));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service.update_email(id, "NEW@example.com").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_email_not_found() {
        let mut mock_repo = MockUserRepo::new();
        let id = Uuid::new_v4();

        mock_repo
            .expect_find_by_email()
            .once()
            .returning(|_| Ok(None));

        mock_repo.expect_update_email().never();

        let service = UserService::new(Arc::new(mock_repo));
        let result = service.update_email(id, "new@example.com").await;

        assert!(matches!(result, Err(Error::User(UserError::UserNotFound))));
    }

    #[tokio::test]
    async fn test_update_password_success() {
        let mut mock_repo = MockUserRepo::new();
        let id = Uuid::new_v4();
        let user = mock_user();

        mock_repo
            .expect_update_password()
            .withf(|_, hash: &str| !hash.is_empty() && hash != "new_password123")
            .once()
            .returning(move |_, _| Ok(user.clone()));

        let service = UserService::new(Arc::new(mock_repo));
        let result = service.update_password(id, "new_password123").await;

        assert!(result.is_ok());
    }
}
