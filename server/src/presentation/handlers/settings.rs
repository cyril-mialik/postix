use actix_web::{HttpResponse, delete, get, patch, post, put, web};

use crate::{
    application::settings::SettingsService,
    domain::entities::{errors::Error, settings::SettingsError},
    presentation::dto::settings::{
        DeleteSettingsRequest, QuerySettingsId, SettingsRequest, SettingsResponse,
        UpdateSettingsFirstNameRequest, UpdateSettingsLastNameRequest, UpdateSettingsRequest,
        UpdateSettingsUsernameRequest,
    },
};

#[get("/settings")]
pub async fn get_settings_by_user_id(
    service: web::Data<SettingsService>,
    query: web::Query<QuerySettingsId>,
) -> Result<HttpResponse, Error> {
    let post = service
        .find_by_user_id(query.into_inner().into())
        .await?
        .ok_or(Error::Settings(SettingsError::NotFound))?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(post)))
}

#[post("/settings")]
pub async fn create_settings(
    service: web::Data<SettingsService>,
    payload: web::Json<SettingsRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.create_settings(payload.into_inner().into()).await?;

    Ok(HttpResponse::Created().json(SettingsResponse::from(post)))
}

#[put("/settings")]
pub async fn update_settings(
    service: web::Data<SettingsService>,
    payload: web::Json<UpdateSettingsRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_settings(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(post)))
}

#[patch("/settings/username")]
pub async fn update_username(
    service: web::Data<SettingsService>,
    payload: web::Json<UpdateSettingsUsernameRequest>,
) -> Result<HttpResponse, Error> {
    let post = service.update_username(payload.into_inner().into()).await?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(post)))
}

#[patch("/settings/first-name")]
pub async fn update_first_name(
    service: web::Data<SettingsService>,
    payload: web::Json<UpdateSettingsFirstNameRequest>,
) -> Result<HttpResponse, Error> {
    let settings = service
        .update_first_name(payload.into_inner().into())
        .await?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(settings)))
}

#[patch("/settings/last-name")]
pub async fn update_last_name(
    service: web::Data<SettingsService>,
    payload: web::Json<UpdateSettingsLastNameRequest>,
) -> Result<HttpResponse, Error> {
    let settings = service
        .update_last_name(payload.into_inner().into())
        .await?;

    Ok(HttpResponse::Ok().json(SettingsResponse::from(settings)))
}

#[delete("/settings")]
pub async fn delete_settings(
    service: web::Data<SettingsService>,
    payload: web::Json<DeleteSettingsRequest>,
) -> Result<HttpResponse, Error> {
    service.delete_post(payload.into_inner().into()).await?;

    Ok(HttpResponse::NoContent().finish())
}
