use actix_web::{HttpResponse, delete, get, patch, post, web};
use uuid::Uuid;

use crate::application::post::PostService;
use crate::domain::entities::errors::Error;
use crate::presentation::dto::post::{
    FeedQuery, PostRequest, PostResponse, PostsResponse, UpdatePostRequest,
};
use crate::presentation::extractors::jwt_auth::AuthenticatedUser;

#[post("/posts")]
pub async fn create_post(
    auth: AuthenticatedUser,
    service: web::Data<PostService>,
    payload: web::Json<PostRequest>,
) -> Result<HttpResponse, Error> {
    let payload = payload.into_inner();
    let post = service
        .create_post(
            auth.user_id,
            payload.title,
            payload.content,
            payload.tag_names,
        )
        .await?;

    Ok(HttpResponse::Created().json(PostResponse::from(post)))
}

#[get("/posts/feed")]
pub async fn get_feed(
    service: web::Data<PostService>,
    query: web::Query<FeedQuery>,
) -> Result<HttpResponse, Error> {
    let posts = service.get_feed(query.limit, query.offset).await?;

    Ok(HttpResponse::Ok().json(PostsResponse::from(posts)))
}

#[get("/posts/{id}")]
pub async fn get_post(
    service: web::Data<PostService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let post = service.get_post(path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[patch("/posts/{id}")]
pub async fn update_post(
    auth: AuthenticatedUser,
    service: web::Data<PostService>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, Error> {
    let payload = payload.into_inner();
    let post = service
        .update_post(
            auth.user_id,
            path.into_inner(),
            payload.title,
            payload.content,
        )
        .await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[delete("/posts/{id}")]
pub async fn delete_post(
    auth: AuthenticatedUser,
    service: web::Data<PostService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    service.delete_post(auth.user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}
