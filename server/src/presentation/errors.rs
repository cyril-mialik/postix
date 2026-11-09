use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;

use crate::domain::entities::{
    auth::AuthError, errors::{Error, SystemError}, follow::FollowError, notification::NotificationError, post::PostError, rating::RatingError, report::ReportError, review::ReviewError, user::UserError,
};

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Auth(e) => auth_error_response(e),
            Error::User(e) => user_error_response(e),
            Error::Post(e) => post_error_response(e),
            Error::Rating(e) => rating_error_response(e),
            Error::Review(e) => review_error_response(e),
            Error::Follow(e) => follow_error_response(e),
            Error::Report(e) => report_error_response(e),
            Error::System(e) => system_error_response(e),
            Error::Notification(e) => notification_error_response(e),
        }
    }

    fn status_code(&self) -> StatusCode {
        self.error_response().status()
    }
}

fn auth_error_response(e: &AuthError) -> HttpResponse {
    let (status, message) = match e {
        AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid credentials"),
        AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid token"),
        AuthError::TokenRevoked => (StatusCode::UNAUTHORIZED, "token revoked"),
        AuthError::Banned(_) => (StatusCode::FORBIDDEN, "user banned"),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}

fn user_error_response(e: &UserError) -> HttpResponse {
    let (status, message) = match e {
        UserError::NotFound => (StatusCode::NOT_FOUND, "user not found"),
        UserError::EmailAlreadyExists => (StatusCode::CONFLICT, "email already exists"),
        UserError::UsernameAlreadyExists => (StatusCode::CONFLICT, "username already exists"),
        UserError::InvalidEmail => (StatusCode::BAD_REQUEST, "invalid email"),
        UserError::InvalidUsernameLength => (StatusCode::BAD_REQUEST, "invalid username length"),
        UserError::Banned => (StatusCode::FORBIDDEN, "user banned"),
        UserError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
        UserError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}

fn post_error_response(e: &PostError) -> HttpResponse {
    let (status, message) = match e {
        PostError::NotFound => (StatusCode::NOT_FOUND, "post not found"),
        PostError::AlreadyExists => (StatusCode::CONFLICT, "post already exists"),
        PostError::InvalidTitleLength => (StatusCode::BAD_REQUEST, "invalid title length"),
        PostError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
        PostError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}

fn rating_error_response(e: &RatingError) -> HttpResponse {
    let (status, message) = match e {
        RatingError::InvalidValue => (StatusCode::BAD_REQUEST, "rating must be 1-5"),
        RatingError::NotFound => (StatusCode::NOT_FOUND, "rating not found"),
        RatingError::AlreadyExists => (StatusCode::CONFLICT, "rating already exists"),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}

fn review_error_response(e: &ReviewError) -> HttpResponse {
    let (status, message) = match e {
        ReviewError::InvalidLength => (StatusCode::BAD_REQUEST, "invalid review length"),
        ReviewError::NotFound => (StatusCode::NOT_FOUND, "review not found"),
        ReviewError::AlreadyExists => (StatusCode::CONFLICT, "review already exists"),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}

fn follow_error_response(e: &FollowError) -> HttpResponse {
    let (status, message) = match e {
        FollowError::CannotFollowSelf => (StatusCode::BAD_REQUEST, "cannot follow yourself"),
        FollowError::AlreadyFollowing => (StatusCode::CONFLICT, "already following"),
        FollowError::NotFound => (StatusCode::NOT_FOUND, "follow not found"),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}

fn report_error_response(e: &ReportError) -> HttpResponse {
    let (status, message) = match e {
        ReportError::NotFound => (StatusCode::NOT_FOUND, "report not found"),
        ReportError::AlreadyExists => (StatusCode::CONFLICT, "report already exists"),
        ReportError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}

fn system_error_response(e: &SystemError) -> HttpResponse {
    let (status, message) = match e {
        SystemError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}

fn notification_error_response(e: &NotificationError) -> HttpResponse {
    let (status, message) = match e {
        NotificationError::NotFound => (StatusCode::NOT_FOUND, "notification not found"),
    };
    HttpResponse::build(status).json(json!({ "error": message }))
}
