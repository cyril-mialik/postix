use actix_web::{HttpResponse, get, patch, web};

use crate::application::profile::ProfileService;
use crate::domain::entities::errors::Error;
use crate::presentation::dto::user::{UpdateProfileRequest, UserResponse};
use crate::presentation::extractors::jwt_auth::AuthenticatedUser;

#[get("/me")]
pub async fn get_me(
    auth: AuthenticatedUser,
    service: web::Data<ProfileService>,
) -> Result<HttpResponse, Error> {
    let user = service.get_profile(auth.user_id).await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[patch("/me")]
pub async fn update_profile(
    auth: AuthenticatedUser,
    service: web::Data<ProfileService>,
    payload: web::Json<UpdateProfileRequest>,
) -> Result<HttpResponse, Error> {
    let payload = payload.into_inner();
    let user = service
        .update_profile(
            auth.user_id,
            payload.username,
            payload.first_name,
            payload.last_name,
            payload.avatar,
        )
        .await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}
