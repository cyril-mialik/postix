use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Неверный email или пароль")]
    InvalidCredentials,

    #[error("Токен недействителен или истек")]
    InvalidToken,

    #[error("Токен отозван")]
    TokenRevoked,
}
