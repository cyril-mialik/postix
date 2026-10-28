use actix_web::{HttpResponse, delete, get, patch, post, put, web};

use crate::{
    application::post::PostService,
    domain::entities::{errors::Error, post::PostError},
    presentation::dto::post::{
        DeletePostRequest, PostRequest, PostResponse, PostsResponse, QueryPostId, QueryPostUserId,
        UpdatePostContentRequest, UpdatePostRequest, UpdatePostTitleRequest,
    },
};

#[get("/post")]
pub async fn get_post_by_id(
    service: web::Data<PostService>,
    query: web::Query<QueryPostId>,
) -> Result<HttpResponse, Error> {
    let post = service
        .find_by_id(query.into_inner().into())
        .await?
        .ok_or(Error::Post(PostError::NotFound))?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[get("/posts")]
pub async fn get_post_by_user_id(
    service: web::Data<PostService>,
    query: web::Query<QueryPostUserId>,
) -> Result<HttpResponse, Error> {
    let posts = service.find_by_user_id(query.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(PostsResponse::from(posts)))
}

#[post("/post")]
pub async fn create_post(
    service: web::Data<PostService>,
    payload: web::Json<PostRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.create_post(payload.into_inner().into()).await?;

    Ok(HttpResponse::Created().json(PostResponse::from(post)))
}

#[put("/post")]
pub async fn update_post(
    service: web::Data<PostService>,
    payload: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_post(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[patch("/post/title")]
pub async fn update_title(
    service: web::Data<PostService>,
    payload: web::Json<UpdatePostTitleRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_title(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[patch("/post/content")]
pub async fn update_content(
    service: web::Data<PostService>,
    payload: web::Json<UpdatePostContentRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_content(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(PostResponse::from(post)))
}

#[delete("/post")]
pub async fn delete_post(
    service: web::Data<PostService>,
    payload: web::Json<DeletePostRequest>,
) -> Result<HttpResponse, Error> {
    service.delete_post(payload.into_inner().into()).await?;

    Ok(HttpResponse::NoContent().finish())
}
