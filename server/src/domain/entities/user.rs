use chrono::{DateTime, Utc};
use derive_builder::{Builder, UninitializedFieldError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

use super::{role::Role, theme::Theme};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: Uuid,

    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub is_email_verified: bool,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
    pub banned_at: Option<DateTime<Utc>>,
    pub role: Role,

    #[validate(length(min = 3, max = 128))]
    pub username: String,
    pub avatar: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub posts_count: i32,
    pub theme: Theme,
    pub deleted_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub const INCREMENT: i32 = 1;
    pub const DECREMENT: i32 = -1;

    pub fn new(dto: CreateUserDto) -> Result<Self, UserError> {
        let user = Self {
            id: Uuid::new_v4(),
            email: dto.email.trim().to_lowercase(),
            password_hash: dto.password_hash,
            is_email_verified: false,
            is_banned: false,
            ban_reason: None,
            banned_at: None,
            role: Role::User,
            username: dto.username.trim().to_owned(),
            avatar: None,
            first_name: dto.first_name.map(|s| s.trim().to_owned()),
            last_name: dto.last_name.map(|s| s.trim().to_owned()),
            followers_count: 0,
            following_count: 0,
            posts_count: 0,
            theme: Theme::Auto,
            deleted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        user.validate().map_err(|e| {
            let field_errors = e.field_errors();
            if field_errors.contains_key("email") {
                return UserError::InvalidEmail;
            }
            if field_errors.contains_key("username") {
                return UserError::InvalidUsernameLength;
            }
            UserError::Validation(e.to_string())
        })?;

        Ok(user)
    }

    pub fn is_active(&self) -> bool {
        self.deleted_at.is_none() && !self.is_banned
    }

    pub fn ensure_can_moderate(&self) -> Result<(), UserError> {
        if !self.role.is_moderator_or_admin() {
            return Err(UserError::Forbidden);
        }
        Ok(())
    }

    pub fn ensure_not_banned(&self) -> Result<(), UserError> {
        if self.is_banned {
            return Err(UserError::Banned);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into), build_fn(error = "CreateUserDtoBuilderError"))]
pub struct CreateUserDto {
    pub email: String,
    pub password_hash: String,
    pub username: String,

    #[builder(default)]
    pub first_name: Option<String>,

    #[builder(default)]
    pub last_name: Option<String>,
}

#[derive(Debug, Error)]
pub enum CreateUserDtoBuilderError {
    #[error(transparent)]
    UninitializedField(#[from] UninitializedFieldError),
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Пользователь не найден")]
    NotFound,

    #[error("Email уже занят")]
    EmailAlreadyExists,

    #[error("Username уже занят")]
    UsernameAlreadyExists,

    #[error("Неверный email")]
    InvalidEmail,

    #[error("Неверная длина username (3–128)")]
    InvalidUsernameLength,

    #[error("Пользователь забанен")]
    Banned,

    #[error("Доступ запрещён")]
    Forbidden,

    #[error("Ошибка валидации: {0}")]
    Validation(String),
}
