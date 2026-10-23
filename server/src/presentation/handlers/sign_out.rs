use actix_web::{HttpRequest, HttpResponse, post, web};

use crate::{
    application::auth::AuthService,
    domain::entities::errors::DomainError,
    infrastructure::security::jwt::{REFRESH_TOKEN_NAME, build_clear_refresh_cookie},
};

#[post("/sign-out")]
pub async fn sign_out(
    service: web::Data<AuthService>,
    req: HttpRequest,
) -> Result<HttpResponse, DomainError> {
    if let Some(cookie) = req.cookie(REFRESH_TOKEN_NAME) {
        service.sign_out(cookie.value()).await?;
    }

    Ok(HttpResponse::NoContent()
        .cookie(build_clear_refresh_cookie())
        .finish())
}
