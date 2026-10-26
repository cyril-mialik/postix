use actix_web::{HttpRequest, HttpResponse, post, web};

use crate::{
    application::auth::AuthService,
    domain::entities::{auth::AuthError, errors::Error},
    infrastructure::security::jwt::{REFRESH_TOKEN_NAME, build_refresh_cookie},
    presentation::dto::auth::AccessTokenResponse,
};

#[post("/refresh")]
pub async fn refresh(
    service: web::Data<AuthService>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let token = req
        .cookie(REFRESH_TOKEN_NAME)
        .map(|c| c.value().to_string())
        .ok_or(Error::Auth(AuthError::InvalidToken))?;

    let tokens = service.refresh(&token).await?;
    let cookie = build_refresh_cookie(&tokens.refresh_token);

    Ok(HttpResponse::Ok().cookie(cookie).json(AccessTokenResponse {
        access_token: tokens.access_token,
    }))
}
