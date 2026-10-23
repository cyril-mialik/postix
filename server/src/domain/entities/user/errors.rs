use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Пользователь с таким email уже существует")]
    EmailAlreadyExists,

    #[error("Пользователь не найден")]
    UserNotFound,

    #[error("Неверный формат email")]
    InvalidEmail,
}
