use crate::domain::entities::{
    auth::AuthError, errors::DomainError, user::UserError,
};
use actix_web::{HttpResponse, ResponseError, http::StatusCode};

impl ResponseError for DomainError {
    fn status_code(&self) -> StatusCode {
        match self {
            DomainError::User(UserError::InvalidEmail) => StatusCode::BAD_REQUEST,
            DomainError::User(UserError::EmailAlreadyExists) => StatusCode::CONFLICT,
            DomainError::User(UserError::UserNotFound) => StatusCode::NOT_FOUND,
            DomainError::Auth(AuthError::InvalidCredentials) => StatusCode::UNAUTHORIZED,
            DomainError::Auth(AuthError::InvalidToken) => StatusCode::UNAUTHORIZED,
            DomainError::Auth(AuthError::TokenRevoked) => StatusCode::UNAUTHORIZED,
            DomainError::System(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}
