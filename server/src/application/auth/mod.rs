use std::sync::Arc;

use crate::domain::{
    entities::{
        auth::errors::AuthError,
        errors::{DomainError, SystemError},
        user::entity::User,
    },
    repositories::{token::TokenRepository, user::repository::UserRepository},
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

    pub async fn sign_in(&self, email: String, password: &str) -> Result<TokenPair, DomainError> {
        let (user, password_hash) = self
            .user_repo
            .find_credentials_by_email(&email)
            .await?
            .ok_or(DomainError::Auth(AuthError::InvalidCredentials))?;

        let is_valid = verify_password(password, &password_hash)
            .map_err(|e| DomainError::System(SystemError::Internal(e.to_string())))?;

        if !is_valid {
            return Err(DomainError::Auth(AuthError::InvalidCredentials));
        }

        self.issue_tokens(&user).await
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenPair, DomainError> {
        let user_id = verify_refresh_token(&self.jwt_secret, refresh_token)
            .map_err(|_| DomainError::Auth(AuthError::InvalidToken))?;

        let is_valid = self
            .token_repo
            .is_refresh_token_valid(user_id, refresh_token)
            .await?;

        if !is_valid {
            return Err(DomainError::Auth(AuthError::TokenRevoked));
        }

        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(DomainError::Auth(AuthError::InvalidToken))?;

        self.token_repo
            .revoke_refresh_token(user_id, refresh_token)
            .await?;

        self.issue_tokens(&user).await
    }

    pub async fn sign_out(&self, refresh_token: &str) -> Result<(), DomainError> {
        let user_id = verify_refresh_token(&self.jwt_secret, refresh_token)
            .map_err(|_| DomainError::Auth(AuthError::InvalidToken))?;

        self.token_repo
            .revoke_refresh_token(user_id, refresh_token)
            .await
    }

    async fn issue_tokens(&self, user: &User) -> Result<TokenPair, DomainError> {
        let access_token = generate_access_token(&self.jwt_secret, user.id)
            .map_err(|e| DomainError::System(SystemError::Internal(e.to_string())))?;

        let refresh_token = generate_refresh_token(&self.jwt_secret, user.id)
            .map_err(|e| DomainError::System(SystemError::Internal(e.to_string())))?;

        self.token_repo
            .save_refresh_token(user.id, &refresh_token, ACCESS_TOKEN_TTL)
            .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}
