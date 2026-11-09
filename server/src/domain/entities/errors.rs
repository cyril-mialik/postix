use thiserror::Error;

use super::{
    auth::AuthError, follow::FollowError, notification::NotificationError, post::PostError,
    rating::RatingError, report::ReportError, review::ReviewError, user::UserError,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Follow(#[from] FollowError),

    #[error(transparent)]
    Notification(#[from] NotificationError),

    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Post(#[from] PostError),

    #[error(transparent)]
    Rating(#[from] RatingError),

    #[error(transparent)]
    Review(#[from] ReviewError),

    #[error(transparent)]
    Report(#[from] ReportError),

    #[error(transparent)]
    System(#[from] SystemError),
}

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Ошибка базы данных: {0}")]
    Database(String),

    #[error("Ошибка кэша: {0}")]
    Cache(String),

    #[error("Ошибка хеширования пароля: {0}")]
    PasswordHashFailed(String),

    #[error("Внутренняя ошибка: {0}")]
    Internal(String),

    #[error("Доступ запрещён")]
    Forbidden,
}
