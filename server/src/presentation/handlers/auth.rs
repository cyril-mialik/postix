use actix_web::{HttpResponse, post, web};
use validator::Validate;

use crate::application::auth::AuthService;
use crate::domain::entities::errors::Error;
use crate::domain::entities::user::UserError;
use crate::infrastructure::security::jwt::{
    ACCESS_TOKEN_TTL, REFRESH_TOKEN_NAME, build_clear_refresh_cookie, build_refresh_cookie,
};
use crate::presentation::dto::auth::{AccessTokenResponse, SignInRequest, SignUpRequest};

#[post("/auth/sign-up")]
pub async fn sign_up(
    service: web::Data<AuthService>,
    payload: web::Json<SignUpRequest>,
) -> Result<HttpResponse, Error> {
    let payload = payload.into_inner();
    validate_auth_request(&payload)?;

    let tokens = service.sign_up(payload.email, &payload.password).await?;

    let cookie = build_refresh_cookie(&tokens.refresh_token);

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(AccessTokenResponse {
            access_token: tokens.access_token,
            token_type: "Bearer".to_string(),
            expires_in: ACCESS_TOKEN_TTL,
        }))
}

#[post("/auth/sign-in")]
pub async fn sign_in(
    service: web::Data<AuthService>,
    payload: web::Json<SignInRequest>,
) -> Result<HttpResponse, Error> {
    let payload = payload.into_inner();
    validate_auth_request(&payload)?;

    let tokens = service.sign_in(payload.email, &payload.password).await?;

    let cookie = build_refresh_cookie(&tokens.refresh_token);

    Ok(HttpResponse::Ok().cookie(cookie).json(AccessTokenResponse {
        access_token: tokens.access_token,
        token_type: "Bearer".to_string(),
        expires_in: ACCESS_TOKEN_TTL,
    }))
}

#[post("/auth/refresh")]
pub async fn refresh(
    service: web::Data<AuthService>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    let token = req
        .cookie(REFRESH_TOKEN_NAME)
        .map(|c| c.value().to_string())
        .ok_or(Error::Auth(
            crate::domain::entities::auth::AuthError::InvalidToken,
        ))?;

    let tokens = service.refresh(&token).await?;
    let cookie = build_refresh_cookie(&tokens.refresh_token);

    Ok(HttpResponse::Ok().cookie(cookie).json(AccessTokenResponse {
        access_token: tokens.access_token,
        token_type: "Bearer".to_string(),
        expires_in: ACCESS_TOKEN_TTL,
    }))
}

#[post("/auth/sign-out")]
pub async fn sign_out(
    service: web::Data<AuthService>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    if let Some(cookie) = req.cookie(REFRESH_TOKEN_NAME) {
        let _ = service.sign_out(cookie.value()).await;
    }

    Ok(HttpResponse::NoContent()
        .cookie(build_clear_refresh_cookie())
        .finish())
}

fn validate_auth_request<T: Validate>(request: &T) -> Result<(), Error> {
    request
        .validate()
        .map_err(|errors| Error::User(UserError::Validation(errors.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_requests_are_validated_before_service_calls() {
        let invalid_sign_up = SignUpRequest {
            email: "reader@example.com".to_owned(),
            password: "short".to_owned(),
        };
        assert!(validate_auth_request(&invalid_sign_up).is_err());

        let invalid_sign_in = SignInRequest {
            email: "not-an-email".to_owned(),
            password: "secret".to_owned(),
        };
        assert!(validate_auth_request(&invalid_sign_in).is_err());

        let valid_sign_up = SignUpRequest {
            email: "writer@example.com".to_owned(),
            password: "long-enough-password".to_owned(),
        };
        assert!(validate_auth_request(&valid_sign_up).is_ok());
    }
}
