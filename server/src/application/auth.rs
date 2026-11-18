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
    jwt::{REFRESH_TOKEN_TTL, generate_access_token, generate_refresh_token, verify_refresh_token},
    password::{hash_password, verify_password},
};

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    token_repo: Arc<dyn TokenRepository>,
    jwt_secret: String,
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

    pub async fn sign_up(&self, email: String, password: &str) -> Result<TokenPair, Error> {
        let password_hash = hash_password(password)
            .map_err(|e| Error::System(SystemError::PasswordHashFailed(e.to_string())))?;

        let username = self.generate_unique_username().await?;

        let dto = crate::domain::entities::user::CreateUserDtoBuilder::default()
            .email(email)
            .password_hash(password_hash)
            .username(username)
            .build()
            .map_err(|e| {
                Error::User(crate::domain::entities::user::UserError::Validation(
                    e.to_string(),
                ))
            })?;

        let user = User::new(dto)?;

        if self.user_repo.find_by_email(&user.email).await?.is_some() {
            return Err(Error::User(
                crate::domain::entities::user::UserError::EmailAlreadyExists,
            ));
        }

        if self
            .user_repo
            .find_by_username(&user.username)
            .await?
            .is_some()
        {
            return Err(Error::User(
                crate::domain::entities::user::UserError::UsernameAlreadyExists,
            ));
        }

        let created = self.user_repo.create(&user).await?;

        self.issue_tokens(&created).await
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

        if user.is_banned {
            return Err(Error::Auth(AuthError::Banned(
                user.ban_reason.clone().unwrap_or_default(),
            )));
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
            .save_refresh_token(user.id, &refresh_token, REFRESH_TOKEN_TTL)
            .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    async fn generate_unique_username(&self) -> Result<String, Error> {
        loop {
            let candidate = User::generate_default_username();

            if self.user_repo.find_by_username(&candidate).await?.is_none() {
                return Ok(candidate);
            }
        }
    }
}
