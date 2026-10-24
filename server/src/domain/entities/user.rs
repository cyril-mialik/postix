use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct User {
    pub id: Uuid,

    #[validate(email)]
    pub email: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String) -> Result<Self, UserError> {
        let email = email.trim().to_lowercase();

        let new_user = Self {
            id: Uuid::new_v4(),
            email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        new_user.validate().map_err(|_| UserError::InvalidEmail)?;

        Ok(new_user)
    }
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Пользователь с таким email уже существует")]
    EmailAlreadyExists,

    #[error("Пользователь не найден")]
    UserNotFound,

    #[error("Неверный формат email")]
    InvalidEmail,
}
