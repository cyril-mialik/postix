use actix_web::{HttpResponse, delete, get, patch, web};

use crate::{
    application::user::UserService,
    domain::entities::{errors::DomainError, user::errors::UserError},
    presentation::{
        dto::user::{UpdateEmailRequest, UpdatePasswordRequest, UserResponse},
        extractors::jwt_auth::AuthenticatedUser,
    },
};

#[get("/me")]
pub async fn get_me(
    auth: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, DomainError> {
    let user = service
        .find_by_id(auth.user_id)
        .await?
        .ok_or(DomainError::User(UserError::UserNotFound))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[patch("/me/email")]
pub async fn update_email(
    auth: AuthenticatedUser,
    service: web::Data<UserService>,
    payload: web::Json<UpdateEmailRequest>,
) -> Result<HttpResponse, DomainError> {
    let user = service
        .update_email(auth.user_id, &payload.new_email)
        .await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[patch("/me/password")]
pub async fn update_password(
    auth: AuthenticatedUser,
    service: web::Data<UserService>,
    payload: web::Json<UpdatePasswordRequest>,
) -> Result<HttpResponse, DomainError> {
    service
        .update_password(auth.user_id, &payload.new_password)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/me")]
pub async fn delete_me(
    auth: AuthenticatedUser,
    service: web::Data<UserService>,
) -> Result<HttpResponse, DomainError> {
    service.delete_user(auth.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
