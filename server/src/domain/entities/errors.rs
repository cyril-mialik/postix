use crate::domain::entities::{
    auth::AuthError, post::PostError, settings::SettingsError, user::UserError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Ошибка базы данных: {0}")]
    Database(String),

    #[error("Внутренняя ошибка: {0}")]
    Internal(String),

    #[error("Ошибка кэша: {0}")]
    Cache(String),

    #[error("Ошибка хэширования пароля")]
    PasswordHashFailed,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    System(#[from] SystemError),

    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Post(#[from] PostError),

    #[error(transparent)]
    Settings(#[from] SettingsError),
}
