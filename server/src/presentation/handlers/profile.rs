use actix_web::{HttpResponse, get, patch, web};

use crate::application::{post::PostService, profile::ProfileService};
use crate::domain::entities::errors::Error;
use crate::presentation::dto::user::{UpdateProfileRequest, UserResponse};
use crate::presentation::extractors::jwt_auth::AuthenticatedUser;

#[get("/me")]
pub async fn get_me(
    auth: AuthenticatedUser,
    service: web::Data<ProfileService>,
    post_service: web::Data<PostService>,
) -> Result<HttpResponse, Error> {
    let user = service.get_profile(auth.user_id).await?;
    let total_views = post_service.total_views_by_author(auth.user_id).await?;
    let mut response = UserResponse::from(user);
    response.total_views_count = total_views;

    Ok(HttpResponse::Ok().json(response))
}

#[patch("/me")]
pub async fn update_profile(
    auth: AuthenticatedUser,
    service: web::Data<ProfileService>,
    post_service: web::Data<PostService>,
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
    let total_views = post_service.total_views_by_author(auth.user_id).await?;
    let mut response = UserResponse::from(user);
    response.total_views_count = total_views;

    Ok(HttpResponse::Ok().json(response))
}
