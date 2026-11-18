use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Неверные учётные данные")]
    InvalidCredentials,

    #[error("Токен недействителен")]
    InvalidToken,

    #[error("Токен отозван")]
    TokenRevoked,

    #[error("Пользователь забанен: {0}")]
    Banned(String),
}
