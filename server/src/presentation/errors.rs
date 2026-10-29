use crate::domain::entities::{
    auth::AuthError, errors::Error, post::PostError, settings::SettingsError, user::UserError,
};
use actix_web::{HttpResponse, ResponseError, http::StatusCode};

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::User(UserError::InvalidEmail) => StatusCode::BAD_REQUEST,
            Error::User(UserError::EmailAlreadyExists) => StatusCode::CONFLICT,
            Error::User(UserError::UserNotFound) => StatusCode::NOT_FOUND,
            Error::Auth(AuthError::InvalidCredentials) => StatusCode::UNAUTHORIZED,
            Error::Auth(AuthError::InvalidToken) => StatusCode::UNAUTHORIZED,
            Error::Auth(AuthError::TokenRevoked) => StatusCode::UNAUTHORIZED,
            Error::Post(PostError::NotFound) => StatusCode::NOT_FOUND,
            Error::Post(PostError::InvalidTitleLength) => StatusCode::BAD_REQUEST,
            Error::Post(PostError::AlreadyExists) => StatusCode::CONFLICT,
            Error::Settings(SettingsError::AlreadyExists) => StatusCode::CONFLICT,
            Error::Settings(SettingsError::NotFound) => StatusCode::NOT_FOUND,
            Error::Settings(SettingsError::InvalidUsernameLength) => StatusCode::BAD_REQUEST,
            Error::Settings(SettingsError::InvalidFirstNameLength) => StatusCode::BAD_REQUEST,
            Error::Settings(SettingsError::InvalidLastNameLength) => StatusCode::BAD_REQUEST,
            Error::System(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}
