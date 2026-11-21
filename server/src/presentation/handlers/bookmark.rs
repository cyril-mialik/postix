use actix_web::{HttpResponse, delete, get, post, web};
use uuid::Uuid;

use crate::{
    application::bookmark::BookmarkService, domain::entities::errors::Error,
    presentation::extractors::jwt_auth::AuthenticatedUser,
};

#[get("/bookmarks")]
pub async fn get_bookmarks(
    auth: AuthenticatedUser,
    service: web::Data<BookmarkService>,
) -> Result<HttpResponse, Error> {
    let bookmarks = service.get_bookmarks(auth.user_id, 100, 0).await?;
    Ok(HttpResponse::Ok().json(bookmarks))
}

#[post("/posts/{id}/bookmark")]
pub async fn bookmark_post(
    auth: AuthenticatedUser,
    service: web::Data<BookmarkService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    service.bookmark(auth.user_id, path.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}

#[delete("/posts/{id}/bookmark")]
pub async fn unbookmark_post(
    auth: AuthenticatedUser,
    service: web::Data<BookmarkService>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    service.unbookmark(auth.user_id, path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
