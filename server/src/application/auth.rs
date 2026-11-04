use std::sync::Arc;

use crate::domain::{
    entities::{
        auth::AuthError,
        errors::{Error, SystemError},
        user::User,
    },
    repositories::{token::TokenRepository, user::UserRepository},
};
use crate::infrastructure::security::{
    jwt::{ACCESS_TOKEN_TTL, generate_access_token, generate_refresh_token, verify_refresh_token},
    password::verify_password,
};

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn TokenRepository>,
    jwt_secret: String,
}

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_repo: Arc<dyn TokenRepository>,
        jwt_secret: String,
    ) -> Self {
        Self {
            user_repo,
            token_repo,
            jwt_secret,
        }
    }

    pub async fn sign_in(&self, email: String, password: &str) -> Result<TokenPair, Error> {
        let (user, password_hash) = self
            .user_repo
            .find_credentials_by_email(&email)
            .await?
            .ok_or(Error::Auth(AuthError::InvalidCredentials))?;

        let is_valid = verify_password(password, &password_hash)
            .map_err(|e| Error::System(SystemError::Internal(e.to_string())))?;

        if !is_valid {
            return Err(Error::Auth(AuthError::InvalidCredentials));
        }

        self.issue_tokens(&user).await
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenPair, Error> {
        let user_id = verify_refresh_token(&self.jwt_secret, refresh_token)
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        let is_valid = self
            .token_repo
            .is_refresh_token_valid(user_id, refresh_token)
            .await?;

        if !is_valid {
            return Err(Error::Auth(AuthError::TokenRevoked));
        }

        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(Error::Auth(AuthError::InvalidToken))?;

        self.token_repo
            .revoke_refresh_token(user_id, refresh_token)
            .await?;

        self.issue_tokens(&user).await
    }

    pub async fn sign_out(&self, refresh_token: &str) -> Result<(), Error> {
        let user_id = verify_refresh_token(&self.jwt_secret, refresh_token)
            .map_err(|_| Error::Auth(AuthError::InvalidToken))?;

        self.token_repo
            .revoke_refresh_token(user_id, refresh_token)
            .await
    }

    async fn issue_tokens(&self, user: &User) -> Result<TokenPair, Error> {
        let access_token = generate_access_token(&self.jwt_secret, user.id)
            .map_err(|e| Error::System(SystemError::Internal(e.to_string())))?;

        let refresh_token = generate_refresh_token(&self.jwt_secret, user.id)
            .map_err(|e| Error::System(SystemError::Internal(e.to_string())))?;

        self.token_repo
            .save_refresh_token(user.id, &refresh_token, ACCESS_TOKEN_TTL)
            .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::{token::TokenRepository, user::UserRepository};
    use chrono::Utc;
    use mockall::mock;
    use mockall::predicate::*;
    use uuid::Uuid;

    mock! {
        pub UserRepo {}

        #[async_trait::async_trait]
        impl UserRepository for UserRepo {
            async fn create_user(&self, user: &User, password_hash: &str) -> Result<User, Error>;
            async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
            async fn find_credentials_by_email(&self, email: &str) -> Result<Option<(User, String)>, Error>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
            async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User, Error>;
            async fn update_password(&self, id: Uuid, new_password: &str) -> Result<User, Error>;
            async fn delete_user(&self, id: Uuid) -> Result<(), Error>;
        }
    }

    mock! {
        pub TokenRepo {}

        #[async_trait::async_trait]
        impl TokenRepository for TokenRepo {
            async fn save_refresh_token(&self, user_id: Uuid, token: &str, ttl_seconds: u64) -> Result<(), Error>;
            async fn is_refresh_token_valid(&self, user_id: Uuid, token: &str) -> Result<bool, Error>;
            async fn revoke_refresh_token(&self, user_id: Uuid, token: &str) -> Result<(), Error>;
            async fn revoke_all_refresh_tokens(&self, user_id: Uuid) -> Result<(), Error>;
        }
    }

    fn mock_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_sign_in_success() {
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_token_repo = MockTokenRepo::new();
        let user = mock_user();
        let user_clone = user.clone();

        let password = "password123";
        let password_hash =
            crate::infrastructure::security::password::hash_password(password).unwrap();

        mock_user_repo
            .expect_find_credentials_by_email()
            .with(eq("test@example.com"))
            .once()
            .returning(move |_| Ok(Some((user_clone.clone(), password_hash.clone()))));

        mock_token_repo
            .expect_save_refresh_token()
            .once()
            .returning(|_, _, _| Ok(()));

        let service = AuthService::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_token_repo),
            "test_secret".to_string(),
        );

        let result = service
            .sign_in("test@example.com".to_string(), password)
            .await;

        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(!tokens.access_token.is_empty());
        assert!(!tokens.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_sign_in_invalid_password() {
        let mut mock_user_repo = MockUserRepo::new();
        let mock_token_repo = MockTokenRepo::new();
        let user = mock_user();

        let password_hash =
            crate::infrastructure::security::password::hash_password("correct_password").unwrap();

        mock_user_repo
            .expect_find_credentials_by_email()
            .once()
            .returning(move |_| Ok(Some((user.clone(), password_hash.clone()))));

        let service = AuthService::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_token_repo),
            "test_secret".to_string(),
        );

        let result = service
            .sign_in("test@example.com".to_string(), "wrong_password")
            .await;

        assert!(matches!(
            result,
            Err(Error::Auth(AuthError::InvalidCredentials))
        ));
    }

    #[tokio::test]
    async fn test_sign_in_user_not_found() {
        let mut mock_user_repo = MockUserRepo::new();
        let mock_token_repo = MockTokenRepo::new();

        mock_user_repo
            .expect_find_credentials_by_email()
            .once()
            .returning(|_| Ok(None));

        let service = AuthService::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_token_repo),
            "test_secret".to_string(),
        );

        let result = service
            .sign_in("notfound@example.com".to_string(), "password")
            .await;

        assert!(matches!(
            result,
            Err(Error::Auth(AuthError::InvalidCredentials))
        ));
    }

    #[tokio::test]
    async fn test_refresh_success() {
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_token_repo = MockTokenRepo::new();
        let user = mock_user();
        let user_id = user.id;
        let user_clone = user.clone();
        let secret = "test_secret".to_string();

        let refresh_token =
            crate::infrastructure::security::jwt::generate_refresh_token(&secret, user_id).unwrap();

        mock_token_repo
            .expect_is_refresh_token_valid()
            .with(eq(user_id), eq(refresh_token.clone()))
            .once()
            .returning(|_, _| Ok(true));

        mock_user_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .once()
            .returning(move |_| Ok(Some(user_clone.clone())));

        mock_token_repo
            .expect_revoke_refresh_token()
            .with(eq(user_id), eq(refresh_token.clone()))
            .once()
            .returning(|_, _| Ok(()));

        mock_token_repo
            .expect_save_refresh_token()
            .once()
            .returning(|_, _, _| Ok(()));

        let service = AuthService::new(Arc::new(mock_user_repo), Arc::new(mock_token_repo), secret);

        let result = service.refresh(&refresh_token).await;

        assert!(result.is_ok());
        let tokens = result.unwrap();
        assert!(!tokens.access_token.is_empty());
        assert!(!tokens.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_refresh_invalid_token() {
        let mock_user_repo = MockUserRepo::new();
        let mock_token_repo = MockTokenRepo::new();

        let service = AuthService::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_token_repo),
            "test_secret".to_string(),
        );

        let result = service.refresh("invalid.token.here").await;

        assert!(matches!(result, Err(Error::Auth(AuthError::InvalidToken))));
    }

    #[tokio::test]
    async fn test_refresh_token_revoked() {
        let mut mock_user_repo = MockUserRepo::new();
        let mut mock_token_repo = MockTokenRepo::new();
        let user = mock_user();
        let user_id = user.id;
        let secret = "test_secret".to_string();

        let refresh_token =
            crate::infrastructure::security::jwt::generate_refresh_token(&secret, user_id).unwrap();

        mock_token_repo
            .expect_is_refresh_token_valid()
            .once()
            .returning(|_, _| Ok(false));

        mock_user_repo.expect_find_by_id().never();

        let service = AuthService::new(Arc::new(mock_user_repo), Arc::new(mock_token_repo), secret);

        let result = service.refresh(&refresh_token).await;

        assert!(matches!(result, Err(Error::Auth(AuthError::TokenRevoked))));
    }

    #[tokio::test]
    async fn test_sign_out_success() {
        let mock_user_repo = MockUserRepo::new();
        let mut mock_token_repo = MockTokenRepo::new();
        let user_id = Uuid::new_v4();
        let secret = "test_secret".to_string();

        let refresh_token =
            crate::infrastructure::security::jwt::generate_refresh_token(&secret, user_id).unwrap();

        mock_token_repo
            .expect_revoke_refresh_token()
            .with(eq(user_id), eq(refresh_token.clone()))
            .once()
            .returning(|_, _| Ok(()));

        let service = AuthService::new(Arc::new(mock_user_repo), Arc::new(mock_token_repo), secret);

        let result = service.sign_out(&refresh_token).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sign_out_invalid_token() {
        let mock_user_repo = MockUserRepo::new();
        let mock_token_repo = MockTokenRepo::new();

        let service = AuthService::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_token_repo),
            "test_secret".to_string(),
        );

        let result = service.sign_out("invalid.token.here").await;

        assert!(matches!(result, Err(Error::Auth(AuthError::InvalidToken))));
    }
}
