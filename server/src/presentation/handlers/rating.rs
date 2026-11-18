use actix_web::{HttpResponse, get, post, web};
use uuid::Uuid;

use crate::{
    application::rating::RatingService,
    domain::entities::errors::Error,
    presentation::{
        dto::rating::{PostRatingRequest, PostRatingSummary},
        extractors::jwt_auth::AuthenticatedUser,
    },
};

#[get("/posts/{post_id}/rating")]
pub async fn get_post_rating(
    auth: AuthenticatedUser,
    service: web::Data<RatingService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let summary: PostRatingSummary = service.summary(path.into_inner(), auth.user_id).await?;
    Ok(HttpResponse::Ok().json(summary))
}

#[post("/posts/{post_id}/rating")]
pub async fn rate_post(
    auth: AuthenticatedUser,
    service: web::Data<RatingService>,
    path: web::Path<Uuid>,
    payload: web::Json<PostRatingRequest>,
) -> Result<HttpResponse, Error> {
    let summary: PostRatingSummary = service
        .rate_post(path.into_inner(), auth.user_id, payload.rating)
        .await?;
    Ok(HttpResponse::Ok().json(summary))
}
