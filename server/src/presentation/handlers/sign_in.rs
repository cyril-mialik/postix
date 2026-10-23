use actix_web::{HttpResponse, post, web};

use crate::{
    application::auth::AuthService,
    domain::entities::errors::DomainError,
    infrastructure::security::jwt::build_refresh_cookie,
    presentation::dto::auth::{AccessTokenResponse, SignInRequest},
};

#[post("/sign-in")]
pub async fn sign_in(
    service: web::Data<AuthService>,
    payload: web::Json<SignInRequest>,
) -> Result<HttpResponse, DomainError> {
    let tokens = service
        .sign_in(payload.email.clone(), &payload.password)
        .await?;

    let cookie = build_refresh_cookie(&tokens.refresh_token);

    Ok(HttpResponse::Ok().cookie(cookie).json(AccessTokenResponse {
        access_token: tokens.access_token,
    }))
}
